use std::process::{self, exit};

use aya::{Ebpf, include_bytes_aligned};
use clap::Parser;
use logger::LoggerConfig;
use nflux_common::dto::Configmap;
use tracing::{error, info, warn};
use utils::{is_true, set_mem_limit};

use crate::{
    cli::NfluxCliArgs, logger::init_logger, tc_program::start_traffic_control, utils::check_is_root,
};

mod cli;
mod events;
mod logger;
mod network_event;
mod tc_program;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = NfluxCliArgs::parse();

    init_logger(LoggerConfig {
        level: cli.log_level,
        format: cli.log_format.clone(),
        with_timer: cli.with_timer,
    });

    if let Err(e) = check_is_root() {
        error!("{e}");
        exit(1);
    }

    set_mem_limit();

    info!("Starting nflux with pid {}", process::id());

    // Match possible subcommands
    match cli.command {
        Some(cli::Commands::Xdp {}) => {
            let mut ebpf_xdp =
                Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/xdp")))?;
            info!("Sniffing xdp packets :)");
            // Uncomment the following line to enable eBPF logging
            if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf_xdp) {
                warn!("failed to initialize eBPF logger: {e}");
            }
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
            disable_arp,
            log_interval,
            disable_full_log,
        }) => {
            let mut ebpf_tc = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/tc")))?;
            // User data shared from the user space to the eBPF program
            info!("Sniffing traffic on interface: {}", interface);

            // Create the configmap for data shared between user space and kernel space
            let configmap = Configmap {
                disable_udp: is_true(disable_udp), // 0 = no, 1 = yes
                disable_icmp: is_true(disable_icmp),
                disable_tcp: is_true(disable_tcp),
                disable_arp: is_true(disable_arp),
                log_interval: log_interval as u64 * 1_000_000_000,
                disable_full_log: is_true(disable_full_log),
                listen_port: listen_port.unwrap_or(0), // Default to 0 if not provided
            };

            // If all protocols are disabled, nothing to do here
            if disable_icmp && disable_tcp && disable_udp && disable_arp {
                error!("You disabled all the protocols (tcp/udp/icmp/arp), nothing to display.");
                exit(1)
            }

            // Start nflux tc
            start_traffic_control(
                &mut ebpf_tc,
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
