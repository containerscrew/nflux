mod logger;

use std::env;
use std::net::Ipv4Addr;

use anyhow::Context;
use aya::{include_bytes_aligned, Ebpf};
use aya::maps::HashMap;
use aya::programs::{Xdp, XdpFlags};
use clap::Parser;
use log::{debug, warn};
use logger::setup_logger;
use tokio::signal;
use tracing::info;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "wlo1")]
    iface: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    // Enable logging
    setup_logger("info".to_string());

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
    // TODO: check if the interface you want to attach is valid (phisical)
    // XDP program can only be attached to physical interfaces
    let program: &mut Xdp = bpf.program_mut("ebpfw").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // Some basic info
    info!("Successfully attached XDP program to iface: {}", &opt.iface);
    info!("Checking incoming packets...");

    // Initialize blocklist
    let mut blocklist: HashMap<_, u32, u32> =
        HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;
    let block_addr: u32 = Ipv4Addr::new(1, 1, 1, 1).try_into()?;
    blocklist.insert(block_addr, 0, 0)?;

    let ctrl_c = signal::ctrl_c();

    info!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    warn!("Exiting...");

    Ok(())
}
