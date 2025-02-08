use std::process;
use std::sync::{Arc, Mutex};
use anyhow::Context;
use aya::{include_bytes_aligned, Ebpf};
use aya::maps::AsyncPerfEventArray;
use aya::util::online_cpus;
use aya_log::EbpfLogger;
use clap::Parser;
use prometheus::Registry;
use tokio::sync::Semaphore;
use tokio::task;
use cli::Cli;
use logger::{setup_logger, LogFormat};
use tracing::{error, info, warn};
use nflux_common::TcConfig;
use traffic_control::start_traffic_control;
use utils::{is_root_user, set_mem_limit, wait_for_shutdown};
use crate::metrics::{start_api, Metrics};
use crate::traffic_control::{process_tc_events};

mod cli;
mod logger;
mod utils;
mod traffic_control;

mod metrics;

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
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Prometheus metrics
    let registry = Registry::new();
    let metrics = Metrics::new(&registry);

    let app_state = Arc::new(Mutex::new(registry.clone()));

    // Start the API in the background
    tokio::spawn(start_api(app_state.clone()));

    let tc_config = TcConfig {
        disable_egress: if cli.disable_private_ips { 1 } else { 0 },
        disable_ingress: if cli.disable_private_ips { 1 } else { 0 },
        disable_private_ips: if cli.disable_private_ips { 1 } else { 0 },
        disable_udp: if cli.disable_udp { 1 } else { 0 },
    };

    // Attach TC program (monitor egress connections)
    start_traffic_control(&mut bpf, cli.interfaces, cli.disable_ingress, cli.disable_egress, tc_config)?;

    let mut tc_events = AsyncPerfEventArray::try_from(
        bpf.take_map("TC_EVENT")
            .context("Failed to find EGRESS_EVENT map")?,
    )?;

    // Spawn tasks for each CPU
    let cpus = online_cpus().map_err(|(_, error)| error)?;
    for cpu_id in cpus {
        let buf = tc_events.open(cpu_id, None)?;
        task::spawn(process_tc_events(buf, cpu_id, metrics.clone()));
    }

    // Wait for shutdown signal
    // This will be removed in future versions, specially for container solution
    wait_for_shutdown().await?;
    Ok(())
}
