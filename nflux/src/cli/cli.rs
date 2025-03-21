use std::process::{self, exit};

use clap::Parser;
use colored::Colorize;
use libc::getuid;
use nflux_common::Configmap;
use tracing::{error, info};

use crate::{logger::init_logger, netrace::start_netrace, utils::{check_is_root_user, is_true, set_mem_limit}};

use super::commands::Commands;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Network monitoring tool & TLS/SSL sniffer using eBPF. Powered by Aya-rs 🐝",
    arg_required_else_help = true,
    before_help = print_banner()
)]
pub struct NfluxCli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(
        short = 'l',
        long = "log-level",
        help = "Log level for logging tracing. Possible values: info, warn, trace, debug, error.",
        default_value = "info",
        required = false
    )]
    pub log_level: String,

    #[arg(
        long = "log-format",
        help = "Log format for logging tracing. Possible values: text, json.",
        default_value = "text",
        required = false
    )]
    pub log_format: String,
}

fn print_banner() -> String {
    r#"
    ███╗   ██╗███████╗██╗     ██╗   ██╗██╗  ██╗
    ████╗  ██║██╔════╝██║     ██║   ██║╚██╗██╔╝
    ██╔██╗ ██║█████╗  ██║     ██║   ██║ ╚███╔╝
    ██║╚██╗██║██╔══╝  ██║     ██║   ██║ ██╔██╗
    ██║ ╚████║██║     ███████╗╚██████╔╝██╔╝ ██╗
    ╚═╝  ╚═══╝╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝
    "#
    .red()
    .to_string()
}


/// start_cli is the main entrypoint of the entire application
pub async fn start_cli() -> Result<NfluxCli, anyhow::Error> {
    let cli = NfluxCli::parse();

    init_logger(&cli.log_level, &cli.log_format.as_str());

    let uid = unsafe { getuid() };
    if let Err(e) = check_is_root_user(uid) {
        error!("{}", e);
        exit(1);
    }

    set_mem_limit();

    match &cli.command {
        Some(Commands::Netrace { interface, enable_egress, enable_ingress, enable_udp, enable_icmp, enable_tcp, log_interval, full_log }
        )=> {
            info!("Starting nflux netrace with pid {}", process::id());
            let configmap = Configmap {
                disable_private_ips: 1, // Not implemented yet
                enable_udp: is_true(*enable_udp), // 0 = no, 1 = yes
                enable_icmp: is_true(*enable_icmp),
                enable_tcp: is_true(*enable_tcp),
                log_interval: *log_interval,
                full_log: is_true(*full_log)
            };

            let _ = start_netrace(interface.as_str(), *enable_egress, *enable_ingress, configmap).await;
        }
        None => {
        }


    }
    Ok(cli)
}
