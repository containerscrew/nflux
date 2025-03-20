use clap::Parser;
use colored::Colorize;

use crate::utils::set_default_iface;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Network monitoring tool & TLS/SSL sniffer using eBPF. Powered by Aya-rs 🐝",
    arg_required_else_help = false,
    before_help = print_banner()
)]
pub struct Cli {
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

    #[arg(
        short = 'i',
        long = "interface",
        help = "Interface to attach the program.",
        default_value_t = set_default_iface(),
        required = false,
    )]
    pub interface: String,

    #[arg(
        long = "enable-egress",
        help = "Enable egress traffic monitoring. [default: true]",
        default_value_t = true,
        required = false
    )]
    pub enable_egress: bool,

    #[arg(
        long = "enable-ingress",
        help = "Enable ingress traffic monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub enable_ingress: bool,

    #[arg(
        long = "enable-udp",
        help = "Enable udp protocol network monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub enable_udp: bool,

    #[arg(
        long = "enable-icmp",
        help = "Enable icmp protocol network monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub enable_icmp: bool,

    #[arg(
        long = "enable-tcp",
        help = "Enable tcp protocol network monitoring. [default: true]",
        default_value_t = true,
        required = false
    )]
    pub enable_tcp: bool,
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
