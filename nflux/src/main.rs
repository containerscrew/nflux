mod config;
mod egress;
mod firewall;
mod logger;
mod utils;

use anyhow::Context;
use aya::maps::AsyncPerfEventArray;
use aya::programs::TcAttachType;
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use std::process;

use crate::egress::process_egress_events;
use config::{Firewall, IsEnabled, Monitoring, Nflux};
use egress::{attach_tc_program, populate_egress_config};
use firewall::{attach_xdp_program, process_firewall_events};
use logger::setup_logger;
use tokio::task;
use tracing::{error, info};
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
    // if let Err(e) = EbpfLogger::init(&mut bpf) {
    //     warn!("failed to initialize eBPF logger: {}", e);
    // }

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

fn start_firewall(bpf: &mut Ebpf, config: Firewall) -> Result<(), anyhow::Error> {
    match config.enabled {
        IsEnabled::True => {
            attach_xdp_program(bpf, config.icmp_ping, &config.rules, &config.interfaces)?;
            info!("Firewall started successfully!");
            print_firewall_rules(config.rules);
        }
        IsEnabled::False => {
            info!("Firewall not enabled");
        }
    }
    Ok(())
}

fn start_traffic_control(bpf: &mut Ebpf, config: Monitoring) -> Result<(), anyhow::Error> {
    match config.enabled {
        IsEnabled::True => {
            if !config.physical_interfaces.is_empty() {
                info!(
                    "Attaching TC egress program to physical interfaces: {:?}",
                    config.physical_interfaces
                );
                attach_tc_program(
                    bpf,
                    "tc_egress_physical",
                    &config.physical_interfaces,
                    TcAttachType::Egress,
                )?;
                attach_tc_program(
                    bpf,
                    "tc_ingress_physical",
                    &config.physical_interfaces,
                    TcAttachType::Ingress,
                )?;
            }

            // Virtual interface is not working fine ATM
            // if !config.virtual_interfaces.is_empty() {
            //     info!(
            //         "Attaching TC egress program to virtual interfaces: {:?}",
            //         config.virtual_interfaces
            //     );
            //     attach_tc_egress_program(bpf, "tc_egress_virtual", &config.virtual_interfaces)?;
            // }
            populate_egress_config(bpf, config)?;
            info!("TC egress started successfully!")
        }
        IsEnabled::False => {
            info!("Egress not enabled");
        }
    }
    Ok(())
}
