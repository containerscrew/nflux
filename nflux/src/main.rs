use std::{process, process::exit};

use aya::{include_bytes_aligned, Ebpf};
use clap::Parser;
use libc::getuid;
use logger::LoggerConfig;
use nflux_common::Configmap;
use tracing::{error, info};
use try_nflux::start_nflux;
use utils::{is_true, set_mem_limit};

use crate::{
    cli::NfluxCliArgs, logger::init_logger, start_dropped_packets::start_dropped_packets,
    utils::is_root_user,
};

mod logger;
mod tc_event;
mod try_nflux;
mod utils;

mod cli;
mod start_dropped_packets;

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

    // Match possible subcommands
    // TODO: refactor this logic matching subcommands
    match cli.command {
        Some(cli::Commands::PktDropped {}) => {
            info!("Sniffing dropped packets");
            start_dropped_packets(&mut ebpf)
                .await
                .expect("Sniffing dropped packets failed");
        }
        None => {}
    }

    // If enable_egress and enable_ingress are both false, the app is doing nothing, exit
    if cli.disable_egress && cli.disable_ingress {
        error!("Can't disable both egress and ingress traffic, nothing to display :)");
        exit(1)
    }

    // Also, if all protocols are disabled, exit
    if cli.disable_icmp && cli.disable_tcp && cli.disable_udp {
        error!("You disabled all the protocols (tcp/udp/icmp), nothing to display :)");
        exit(1)
    }

    // Prepare configmap data (data from user space to be processed in kernel space using eBPF maps)
    let configmap = Configmap {
        disable_udp: is_true(cli.disable_udp), // 0 = no, 1 = yes
        disable_icmp: is_true(cli.disable_icmp),
        disable_tcp: is_true(cli.disable_tcp),
        log_interval: cli.log_interval as u64 * 1_000_000_000,
        disable_full_log: is_true(cli.disable_full_log),
        listen_port: cli.listen_port.unwrap_or(0), // Default to 0 if not provided
    };

    // Start nflux
    start_nflux(
        &mut ebpf,
        &cli.interface,
        cli.disable_egress,
        cli.disable_ingress,
        configmap,
        cli.log_format,
        cli.exclude_port,
    )
    .await
    .expect("Failed to start nflux");
    Ok(())
}
