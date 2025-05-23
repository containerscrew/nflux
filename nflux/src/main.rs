use std::{process, process::exit};

use clap::Parser;
use libc::getuid;
use nflux_common::Configmap;
use tracing::{error, info};
use try_nflux::start_nflux;
use utils::{is_true, set_mem_limit};

use crate::{cli::NfluxCliArgs, logger::init_logger, utils::check_is_root_user};

mod logger;
mod tc_event;
mod try_nflux;
mod utils;

mod cli;

#[tokio::main]
async fn main() {
    // Parse cli args
    let cli = NfluxCliArgs::parse();

    // Start logger
    init_logger(&cli.log_level, &cli.log_format.as_str());

    // User should be root
    let uid = unsafe { getuid() };
    if let Err(e) = check_is_root_user(uid) {
        error!("{}", e);
        exit(1);
    }

    // Set memory limit for eBPF maps
    set_mem_limit();

    // Welcome message
    info!("Starting nflux with pid {}", process::id());

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
        disable_private_ips: 1,                // Not implemented yet
        disable_udp: is_true(cli.disable_udp), // 0 = no, 1 = yes
        disable_icmp: is_true(cli.disable_icmp),
        disable_tcp: is_true(cli.disable_tcp),
        log_interval: cli.log_interval,
        disable_full_log: is_true(cli.disable_full_log),
    };

    // Start nflux
    start_nflux(
        &cli.interface,
        cli.disable_egress,
        cli.disable_ingress,
        configmap,
    )
    .await
    .expect("Failed to start nflux");
}
