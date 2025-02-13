use std::process;

use aya::{include_bytes_aligned, maps::RingBuf, Ebpf};
use clap::Parser;
use cli::Cli;
use logger::{setup_logger, LogFormat};
use nflux_common::{utils::is_true, TcConfig};
use tc::{process_event, start_traffic_control};
use tracing::{error, info};
use utils::{is_root_user, set_mem_limit, wait_for_shutdown};

mod cli;
mod logger;
mod tc;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start CLI
    let cli = Cli::parse();

    // Enable logging
    setup_logger(&cli.log_level, LogFormat::Text);

    // Ensure the program is run as root
    if !is_root_user() {
        error!("This program must be run as root.");
        process::exit(1);
    }

    // Welcome message
    info!("Starting nflux with pid {}", process::id());

    // Set memory limit
    set_mem_limit();

    // Load eBPF program
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    // Necessary to debug something in the ebpf code
    // if let Err(e) = EbpfLogger::init(&mut bpf) {
    //     warn!("failed to initialize eBPF logger: {}", e);
    // }

    let tc_config = TcConfig {
        disable_egress: is_true(cli.disable_egress),
        enable_ingress: is_true(cli.enable_ingress),
        disable_private_ips: is_true(cli.disable_private_ips),
        enable_udp: is_true(cli.enable_udp),
    };

    // Attach TC program to interfaces
    start_traffic_control(
        &mut bpf,
        cli.interfaces,
        cli.enable_ingress,
        cli.disable_egress,
        tc_config,
    )?;

    // Traffic control event ring buffer
    let tc_event_ring_map = bpf
        .take_map("TC_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find TC_EVENT map"))?;

    let ring_buf = RingBuf::try_from(tc_event_ring_map)?;

    // Spawn a task to process events
    tokio::spawn(async move { process_event(ring_buf).await });

    // Wait for shutdown
    let _ = wait_for_shutdown().await;

    Ok(())
}
