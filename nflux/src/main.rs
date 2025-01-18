mod config;
mod egress;
mod firewall;
mod logger;
mod utils;

use anyhow::Context;
use aya::maps::AsyncPerfEventArray;
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use aya_log::EbpfLogger;
use std::process;

use crate::egress::process_egress_events;
use config::{IsEnabled, Nflux};
use egress::{attach_tc_egress_program, populate_egress_config};
use firewall::{attach_xdp_program, process_firewall_events};
use logger::setup_logger;
use tokio::task;
use tracing::{error, info, warn};
use utils::{is_root_user, print_firewall_rules, set_mem_limit, wait_for_shutdown};

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
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Attach XDP program (monitor ingress connections to local ports)
    match config.firewall.enabled {
        IsEnabled::True => {
            attach_xdp_program(
                &mut bpf,
                config.firewall.icmp_ping,
                &config.firewall.rules,
                &config.firewall.interfaces,
            )?;
            info!("firewall started successfully!");
            print_firewall_rules(config.firewall.rules);
        }
        IsEnabled::False => {
            info!("Firewall not enabled")
        }
    }

    // Attach TC program (monitor egress connections)
    match config.egress.enabled {
        IsEnabled::True => {
            if !config.egress.physical_interfaces.is_empty() {
                info!(
                    "Attaching TC egress program to physical interfaces: {:?}",
                    config.egress.physical_interfaces
                );
                attach_tc_egress_program(
                    &mut bpf,
                    "tc_egress_physical",
                    &config.egress.physical_interfaces,
                )?;
            }

            if !config.egress.virtual_interfaces.is_empty() {
                info!(
                    "Attaching TC egress program to virtual interfaces: {:?}",
                    config.egress.virtual_interfaces
                );
                attach_tc_egress_program(
                    &mut bpf,
                    "tc_egress_virtual",
                    &config.egress.virtual_interfaces,
                )?;
            }
            populate_egress_config(&mut bpf, config.egress)?;
            info!("TC egress started successfully!")
        }
        IsEnabled::False => {
            info!("Egress not enabled");
        }
    }

    // Start processing events from the eBPF program
    let mut firewall_events = AsyncPerfEventArray::try_from(
        bpf.take_map("FIREWALL_EVENTS")
            .context("Failed to find CONNECTION_EVENTS map")?,
    )?;

    let mut egress_events = AsyncPerfEventArray::try_from(
        bpf.take_map("EGRESS_EVENT")
            .context("Failed to find EGRESS_EVENT map")?,
    )?;

    let cpus = online_cpus().map_err(|(_, error)| error)?;
    for cpu_id in cpus {
        // Spawn task for firewall events
        {
            let buf = firewall_events.open(cpu_id, None)?;
            task::spawn(process_firewall_events(buf, cpu_id));
        }

        // Spawn task for egress events
        {
            let buf = egress_events.open(cpu_id, None)?;
            task::spawn(process_egress_events(buf, cpu_id));
        }
    }

    // Wait for shutdown signal
    // This will removed in future versions, specially for container solution
    wait_for_shutdown().await?;
    Ok(())
}
