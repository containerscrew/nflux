use std::process::{self, exit};

use clap::Parser;
use colored::Colorize;
use libc::getuid;
use tracing::{error, info};

use crate::{custom_logger::setup_logger, utils::{check_is_root_user, set_mem_limit}};

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
pub fn start_cli() -> Result<NfluxCli, anyhow::Error> {
    let cli = NfluxCli::parse();

    setup_logger(&cli.log_level, &cli.log_format.as_str());

    let uid = unsafe { getuid() };
    if let Err(e) = check_is_root_user(uid) {
        error!("{}", e);
        exit(1);
    }

    set_mem_limit();

    match &cli.command {
        Some(Commands::Netrace { interface, enable_egress, enable_ingress, enable_udp, enable_icmp, enable_tcp }
        )=> {
            info!("Starting nflux netrace with pid {}", process::id());
        }
        None => {
        }


    }
    Ok(cli)
}
