use std::{process, process::exit};

use aya::{include_bytes_aligned, Ebpf};
use clap::Parser;
use libc::getuid;
use logger::LoggerConfig;
use nflux_common::Configmap;
use tracing::{error, info};
use utils::{is_true, set_mem_limit};

use crate::{
    cli::NfluxCliArgs,
    logger::init_logger,
    programs::{start_dropped_packets, start_traffic_control},
    utils::is_root_user,
};

mod cli;
mod events;
mod logger;
mod programs;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = NfluxCliArgs::parse();

    init_logger(LoggerConfig {
        level: cli.log_level,
        format: cli.log_format.clone(),
        with_timer: cli.with_timer,
    });

    let uid = unsafe { getuid() };
    if let Err(e) = is_root_user(uid) {
        error!("{}", e);
        exit(1);
    }

    // Set memory limit for eBPF maps
    set_mem_limit();

    info!("Starting nflux with pid {}", process::id());

    // Load eBPF program
    let mut ebpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    // if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
    //     // This can happen if you remove all log statements from your eBPF program.
    //     warn!("failed to initialize eBPF logger: {e}");
    // }

    // Match possible subcommands
    match cli.command {
        Some(cli::Commands::Dpkt {}) => {
            info!("Sniffing dropped packets");
            start_dropped_packets(&mut ebpf, cli.log_format).await?;
        }
        Some(cli::Commands::Tc {
            interface,
            disable_egress,
            disable_ingress,
            listen_port,
            exclude_port,
            disable_udp,
            disable_icmp,
            disable_tcp,
            log_interval,
            disable_full_log,
        }) => {
            info!("Sniffing traffic on interface: {}", interface);
            let configmap = Configmap {
                disable_udp: is_true(disable_udp), // 0 = no, 1 = yes
                disable_icmp: is_true(disable_icmp),
                disable_tcp: is_true(disable_tcp),
                log_interval: log_interval as u64 * 1_000_000_000,
                disable_full_log: is_true(disable_full_log),
                listen_port: listen_port.unwrap_or(0), // Default to 0 if not provided
            };

            // If enable_egress and enable_ingress are both false, the app is doing nothing, exit
            if disable_egress && disable_ingress {
                error!("Can't disable both egress and ingress traffic, nothing to display.");
                exit(1)
            }

            // Also, if all protocols are disabled, exit
            if disable_icmp && disable_tcp && disable_udp {
                error!("You disabled all the protocols (tcp/udp/icmp), nothing to display.");
                exit(1)
            }

            // Start nflux tc
            start_traffic_control(
                &mut ebpf,
                &interface,
                disable_egress,
                disable_ingress,
                configmap,
                cli.log_format,
                exclude_port,
            )
            .await?;
        }
        None => {
            // Unreachable: CLI shows help if no args are provided.
        }
    }

    Ok(())
}
