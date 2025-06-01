use std::fmt;

use clap::Parser;
use colored::Colorize;

use crate::utils::set_default_iface;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Network monitoring tool & TLS/SSL sniffer using eBPF 🐝\nAuthor: github.com/containerscrew",
    arg_required_else_help = false,
    before_help = print_banner()
)]
pub struct NfluxCliArgs {
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
        long = "disable-egress",
        help = "Disable egress traffic monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub disable_egress: bool,

    #[arg(
        long = "disable-ingress",
        help = "Disable ingress traffic monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub disable_ingress: bool,

    #[arg(
        long = "disable-udp",
        help = "Disable udp protocol network monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub disable_udp: bool,

    #[arg(
        long = "disable-icmp",
        help = "Disable icmp protocol network monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub disable_icmp: bool,

    #[arg(
        long = "disable-tcp",
        help = "Disable tcp protocol network monitoring. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub disable_tcp: bool,

    #[arg(
        long = "log-interval",
        help = "Enable tcp protocol network monitoring. This flag is not used if you don't set --disable-full-log. [default: 5(seconds)]",
        default_value_t = 5,
        required = false
    )]
    pub log_interval: u8,

    #[arg(
        long = "disable-full-log",
        help = "Disable log for every packet. Then use the flag --log-interval Xs. [default: false]",
        default_value_t = false,
        required = false
    )]
    pub disable_full_log: bool,

    #[arg(
        long = "with-timer",
        help = "Add timer to the logs. This will add a timestamp to each log entry. Ej: 2025-06-01 23:02:47",
        default_value_t = false,
        required = false
    )]
    pub with_timer: bool,
}

// Custom implementation of Display trait for the Structure NfluxCliArgs
impl fmt::Display for NfluxCliArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "log_level: {}, log_format: {}, interface: {}, \
             disable_egress: {}, disable_ingress: {}, disable_udp: {}, disable_icmp: {}, \
             disable_tcp: {}, log_interval: {}, disable_full_log: {}",
            self.log_level,
            self.log_format,
            self.interface,
            self.disable_egress,
            self.disable_ingress,
            self.disable_udp,
            self.disable_icmp,
            self.disable_tcp,
            self.log_interval,
            self.disable_full_log
        )
    }
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
