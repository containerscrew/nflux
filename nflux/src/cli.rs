use clap::Parser;

#[derive(Parser, Debug)]

#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Network monitoring tool using eBPF. Powered by Aya-rs 🐝",
    arg_required_else_help = false
)]

pub struct Cli {
    #[arg(
        short = 'l',
        long = "log-level",
        help = "Log level for logging tracing. Possible values: info, warn, trace, debug, error. Default: info",
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
        required = true
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

    #[arg(
        long = "log-every",
        help = "Log every N seconds. Do not log every packet for the same pid every N seconds.",
        default_value_t = 5,
        required = false
    )]
    pub log_every: u32,
}
