mod logger;
mod config;
mod utils;

use std::env;
use anyhow::Context;
use aya::{include_bytes_aligned, Ebpf};
use aya::maps::{Array};
use aya::programs::{Xdp, XdpFlags};
use log::{debug, warn};
use logger::setup_logger;
use tokio::signal;
use tracing::{error, info};
use ebpfw_common::MAX_ALLOWED_PORTS;
use crate::config::Config;
use crate::utils::is_root_user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration file. Set the CONFIG_FILE_PATH env var. Example: CONFIG_FILE_PATH=./config.toml
    let config = Config::load_config();

    // Enable logging
    setup_logger(config.log.log_level);

    // Check if user is root.
    if !is_root_user() {
        error!("This program must be run as root.");
        std::process::exit(1);
    }

    info!("starting ebpfw");

    // Bump the memlock rlimit
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // Load eBPF program
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/ebpfw")))?;
    if let Err(e) = aya_log::EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Attach XDP program
    // TODO: check if the interface you want to attach is valid (physical)
    // XDP program can only be attached to physical interfaces
    let program: &mut Xdp = bpf.program_mut("ebpfw").unwrap().try_into()?;
    program.load()?;
    program.attach(config.ebpfw.interface_name.as_str(), XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // Some basic info
    info!("Successfully attached XDP program to iface: {}", config.ebpfw.interface_name);
    info!("Checking incoming packets...");

    // Allowed list
    // Set up the allowed ports array
    let mut allowed_ports: Array<_, u32> = Array::try_from(bpf.map_mut("ALLOWED_PORTS").unwrap())?;

    // Iterate over the vector and add each port to the eBPF array
    for (index, &port) in config.firewall.allowed_ports.iter().enumerate() {
        if index < MAX_ALLOWED_PORTS {
            allowed_ports
                .set(index as u32, port as u32, 0)
                .context(format!("Failed to set port {} in the allowed ports list", port))?;
        } else {
            warn!("Skipping port {} because the maximum allowed ports limit was reached", port);
        }
    }

    info!("Allowed ports: {:?}", config.firewall.allowed_ports);

    let ctrl_c = signal::ctrl_c();
    info!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    warn!("Exiting...");

    Ok(())
}
