mod config;
mod logger;
mod utils;
mod firewall;

use anyhow::Context;
use aya::maps::AsyncPerfEventArray;
use aya::programs::{Xdp, XdpFlags};
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use aya_log::EbpfLogger;

use config::{IsEnabled, Nflux};
use firewall::{populate_ip_rules, process_firewall_events};
use log::warn;
use logger::setup_logger;
use tokio::task;
use tracing::{error, info};
use utils::{is_root_user, print_firewall_rules, set_mem_limit, wait_for_shutdown};
use crate::firewall::populate_icmp_rule;

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

    // Set memory limit
    set_mem_limit();

    // Load eBPF program
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    // Necessary to debug something in the ebpf code
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Attach XDP program
    match config.firewall.enabled {
        IsEnabled::True => {
            // Populate eBPF maps with configuration data
            populate_ip_rules(&mut bpf, &config.firewall_rules)?;
            populate_icmp_rule(&mut bpf, config.firewall.icmp_ping)?;

            // Load the XDP program
            let program: &mut Xdp = bpf.program_mut("xdp_firewall").unwrap().try_into()?;
            program.load()?;

            // Attach the XDP program to multiple interfaces
            for interface in &config.firewall.interfaces {
                if let Err(e) = program.attach(interface, XdpFlags::default()) {
                    error!(
                        "Failed to attach XDP program to interface {}: {}. Ensure it is a physical interface.",
                        interface, e
                    );
                } else {
                    info!("XDP program attached to interface: {}", interface);
                }
            }
        }
        IsEnabled::False => {
            // If not enabled, just log
            info!("Firewall not enabled")
        }
    }


    // Attach TC program
    // match config.egress.enabled {
    //     IsEnabled::True => {
    //         // Add clsact qdisc
    //         if let Err(e) = tc::qdisc_add_clsact(&config.egress.interface_name) {
    //             warn!(
    //                 "Failed to add clsact qdisc to interface {:?}: {:?}",
    //                 config.egress.interface_name, e
    //             );
    //         }

    //         // Get the tc_egress program
    //         let program = match bpf.program_mut("tc_egress") {
    //             Some(p) => p,
    //             None => {
    //                 error!("Failed to find the tc_egress program in BPF object");
    //                 return Err(anyhow::anyhow!("tc_egress program not found"));
    //             }
    //         };

    //         // Try converting the program into a SchedClassifier
    //         let program: &mut SchedClassifier = match program.try_into() {
    //             Ok(p) => p,
    //             Err(e) => {
    //                 error!("Failed to convert tc_egress program to SchedClassifier: {:?}", e);
    //                 return Err(e.into());
    //             }
    //         };

    //         // Load the program
    //         if let Err(e) = program.load() {
    //             error!("Failed to load tc_egress program: {:?}", e);
    //             return Err(e.into());
    //         }

    //         // Attach the program
    //         if let Err(e) = program.attach(&config.egress.interface_name, TcAttachType::Egress) {
    //             error!(
    //                 "Failed to attach tc_egress program to interface {:?}: {:?}",
    //                 config.egress.interface_name, e
    //             );
    //             return Err(e.into());
    //         }

    //         info!(
    //             "TC egress program successfully attached to interface: {:?}",
    //             config.egress.interface_name
    //         );
    //     }
    //     IsEnabled::False => {
    //         info!("Egress not enabled");
    //     }
    // }

    // Log startup info
    info!("nflux started successfully!");
    print_firewall_rules(config.firewall_rules);

    // Start processing events from the eBPF program
    let mut firewall_events = AsyncPerfEventArray::try_from(
        bpf.take_map("FIREWALL_EVENTS")
            .context("Failed to find CONNECTION_EVENTS map")?,
    )?;

    // let mut egress_events = AsyncPerfEventArray::try_from(
    //     bpf.take_map("EGRESS_EVENT")
    //         .context("Failed to find EGRESS_EVENT map")?,
    // )?;

    let cpus = online_cpus().map_err(|(_, error)| error)?;

    for cpu_id in cpus {
        // Spawn task for connection events
        {
            let buf = firewall_events.open(cpu_id, None)?;
            task::spawn(process_firewall_events(buf, cpu_id));
        }

        // // Spawn task for egress events
        // {
        //     let buf = egress_events.open(cpu_id, None)?;
        //     task::spawn(process_egress_events(buf, cpu_id));
        // }
    }

    // Wait for shutdown signal
    wait_for_shutdown().await?;
    Ok(())
}
