use std::process;

use aya::{include_bytes_aligned, Ebpf};
use clap::Parser;
use cli::Cli;
use logger::{setup_logger, LogFormat};
use tracing::{error, info};
use traffic_control::start_traffic_control;
use utils::{is_root_user, set_mem_limit, wait_for_shutdown};


mod cli;
mod logger;
mod utils;
mod traffic_control;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start CLI
    let cli = Cli::parse();

    // Enable logging
    setup_logger(&cli.log_level, LogFormat::Text);

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

    // // Necessary to debug something in the ebpf code
    // // if let Err(e) = EbpfLogger::init(&mut bpf) {
    // //     warn!("failed to initialize eBPF logger: {}", e);
    // // }

    // Prometheus metrics
    // let registry = Registry::new();
    // let metrics = Metrics::new(&registry);

    // let app_state = Arc::new(Mutex::new(registry.clone()));

    // // Start the API in the background
    // tokio::spawn(start_api(app_state.clone()));

    // Attach TC program (monitor egress connections)
    start_traffic_control(&mut bpf, cli.interfaces)?;

    // // Start processing events from the eBPF program
    // let mut firewall_events = AsyncPerfEventArray::try_from(
    //     bpf.take_map("FIREWALL_EVENTS")
    //         .context("Failed to find CONNECTION_EVENTS map")?,
    // )?;

    // let mut egress_events = AsyncPerfEventArray::try_from(
    //     bpf.take_map("EGRESS_EVENT")
    //         .context("Failed to find EGRESS_EVENT map")?,
    // )?;

    // // Spawn tasks for each CPU
    // let cpus = online_cpus().map_err(|(_, error)| error)?;
    // for cpu_id in cpus {
    //     // Spawn task for xdp_firewall events
    //     {
    //         let buf = firewall_events.open(cpu_id, None)?;
    //         task::spawn(process_firewall_events(buf, cpu_id));
    //     }

    //     // Spawn task for traffic control events
    //     {
    //         let buf = egress_events.open(cpu_id, None)?;
    //         task::spawn(process_egress_events(buf, cpu_id, metrics.clone()));
    //     }
    // }

    // // Wait for shutdown signal
    // // This will be removed in future versions, specially for container solution
    wait_for_shutdown().await?;
    Ok(())
}
