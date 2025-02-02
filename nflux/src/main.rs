mod config;
mod traffic_control;
mod xdp_firewall;
mod logger;
mod utils;
mod prometheus;

use anyhow::Context;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use aya::maps::AsyncPerfEventArray;
use aya::programs::TcAttachType;
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use traffic_control::{attach_tc_program, populate_egress_config};
use prometheus::Metrics;
use ::prometheus::{Encoder, Registry, TextEncoder};
use std::net::SocketAddr;
use std::process;
use std::sync::{Arc, Mutex};

use crate::traffic_control::{process_egress_events, start_traffic_control};
use config::{Firewall, IsEnabled, Monitoring, Nflux};
use xdp_firewall::{attach_xdp_program, process_firewall_events};
use logger::setup_logger;
use tokio::task;
use tracing::{error, info};
use utils::{is_root_user, print_firewall_rules, set_mem_limit, wait_for_shutdown};
use crate::prometheus::start_api;
use crate::xdp_firewall::start_firewall;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration file
    let config = Nflux::load_config().context("Failed to load nflux configuration")?;

    // Enable logging
    setup_logger(&config.logging.log_level, &config.logging.log_type);

    // Ensure the program is run as root
    if !is_root_user() {
        error!("This program must be run as root.");
        std::process::exit(1);
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

    // Prometheus metrics
    let registry = Registry::new();
    let metrics = Metrics::new(&registry);

    let app_state = Arc::new(Mutex::new(registry.clone()));

    // Start the API in the background
    tokio::spawn(start_api(app_state.clone()));

    // Attach XDP program (monitor ingress connections to local ports)
    start_firewall(&mut bpf, config.firewall)?;

    // Attach TC program (monitor egress connections)
    start_traffic_control(&mut bpf, config.monitoring)?;

    // Start processing events from the eBPF program
    let mut firewall_events = AsyncPerfEventArray::try_from(
        bpf.take_map("FIREWALL_EVENTS")
            .context("Failed to find CONNECTION_EVENTS map")?,
    )?;

    let mut egress_events = AsyncPerfEventArray::try_from(
        bpf.take_map("EGRESS_EVENT")
            .context("Failed to find EGRESS_EVENT map")?,
    )?;

    // Spawn tasks for each CPU
    let cpus = online_cpus().map_err(|(_, error)| error)?;
    for cpu_id in cpus {
        // Spawn task for xdp_firewall events
        {
            let buf = firewall_events.open(cpu_id, None)?;
            task::spawn(process_firewall_events(buf, cpu_id));
        }

        // Spawn task for traffic control events
        {
            let buf = egress_events.open(cpu_id, None)?;
            task::spawn(process_egress_events(buf, cpu_id, metrics.clone()));
        }
    }

    // Wait for shutdown signal
    // This will be removed in future versions, specially for container solution
    wait_for_shutdown().await?;
    Ok(())
}
