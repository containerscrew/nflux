use clap::Parser;
use colored::Colorize;

use crate::utils::set_default_iface;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Network monitoring tool using eBPF. Powered by Aya-rs 🐝",
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
        short = 'i',
        long = "interfaces",
        help = "List of interfaces to attach the program",
        value_delimiter = ' ',
        num_args = 1..,
        default_values_t = set_default_iface(),
        required = false,
    )]
    pub interfaces: Vec<String>,

    #[arg(
        long = "disable-egress",
        help = "Disable egress",
        default_value_t = false,
        required = false
    )]
    pub disable_egress: bool,

    #[arg(
        long = "enable-ingress",
        help = "Enable ingress traffic monitoring",
        default_value_t = false,
        required = false
    )]
    pub enable_ingress: bool,

    #[arg(
        long = "disable-private-ips",
        help = "Disable private ips network monitoring",
        default_value_t = true,
        required = false
    )]
    pub disable_private_ips: bool,

    #[arg(
        long = "enable-udp",
        help = "Enable udp network monitoring",
        default_value_t = false,
        required = false
    )]
    pub enable_udp: bool,
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
