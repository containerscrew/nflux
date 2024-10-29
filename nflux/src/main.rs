mod cli;
mod config;
mod core;
mod logger;
mod utils;

use crate::cli::Args;
use crate::config::Config;
use crate::core::set_mem_limit;
use crate::utils::{is_root_user, wait_for_shutdown};
use anyhow::Context;
use aya::maps::{Array, AsyncPerfEventArray};
use aya::programs::{Xdp, XdpFlags};
use aya::{include_bytes_aligned, Ebpf};
use bytes::BytesMut;
use clap::Parser;
use log::warn;
use logger::setup_logger;
use nflux_common::MAX_ALLOWED_PORTS;
use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::{signal, task};
use tracing::{error, info};

#[repr(C)]
struct ConnectionEvent {
    src_addr: u32,
    dst_addr: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8, // 6 for TCP, 17 for UDP
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Load configuration file. Set the CONFIG_FILE_PATH env var. Example: CONFIG_FILE_PATH=./config.toml
    let config = Config::load_config(args.config_file.as_str());

    // Enable logging
    setup_logger(&config.log.log_level);

    // Check if user is root.
    if !is_root_user() {
        error!("This program must be run as root.");
        std::process::exit(1);
    }

    // Mem limit
    set_mem_limit();

    // Load eBPF program
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;
    if let Err(e) = aya_log::EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Attach XDP program
    // TODO: check if the interface you want to attach is valid (physical)
    // XDP program can only be attached to physical interfaces
    let program: &mut Xdp = bpf.program_mut("nflux").unwrap().try_into()?;
    program.load()?;
    program.attach(config.nflux.interface_name.as_str(), XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // Some basic info
    info!("starting nflux");
    info!(
        "Successfully attached XDP program to iface: {}",
        config.nflux.interface_name
    );
    info!("Checking incoming packets...");

    // Populate allowed IPv4 addresses
    populate_allowed_ipv4(&mut bpf, &config)?;

    // Populate allowed ports
    populate_allowed_ports(&mut bpf, &config)?;

    // Wait for shutdown signal
    wait_for_shutdown().await?;

    Ok(())
}

// Populate the eBPF array map for allowed IPv4 addresses
fn populate_allowed_ipv4(bpf: &mut Ebpf, config: &Config) -> anyhow::Result<()> {
    let mut allowed_ipv4: Array<_, u32> = Array::try_from(bpf.map_mut("ALLOWED_IPV4").unwrap())?;
    for (index, ip_str) in config.firewall.allowed_ipv4.iter().enumerate() {
        let ip: Ipv4Addr = Ipv4Addr::from_str(ip_str)
            .with_context(|| format!("Invalid IP address: {}", ip_str))?;
        allowed_ipv4
            .set(index as u32, u32::from(ip), 0)
            .with_context(|| format!("Failed to set IP address {} in eBPF map", ip_str))?;
    }
    info!("Allowed IPv4 addresses: {:?}", config.firewall.allowed_ipv4);
    Ok(())
}

// Populate the eBPF array map for allowed ports
fn populate_allowed_ports(bpf: &mut Ebpf, config: &Config) -> anyhow::Result<()> {
    let mut allowed_ports: Array<_, u32> = Array::try_from(bpf.map_mut("ALLOWED_PORTS").unwrap())?;
    for (index, &port) in config.firewall.allowed_ports.iter().enumerate() {
        if index < MAX_ALLOWED_PORTS {
            allowed_ports
                .set(index as u32, port as u32, 0)
                .context(format!(
                    "Failed to set port {} in the allowed ports list",
                    port
                ))?;
        } else {
            warn!(
                "Skipping port {} because the maximum allowed ports limit was reached",
                port
            );
        }
    }
    info!("Allowed ports: {:?}", config.firewall.allowed_ports);
    Ok(())
}
