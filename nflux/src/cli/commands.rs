use clap::Subcommand;

use crate::utils::set_default_iface;


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start network traffic monitoring using TC (Traffic Control)
    Netrace {
        #[arg(
            short = 'i',
            long = "interface",
            help = "Interface to attach the program.",
            default_value_t = set_default_iface(),
            required = false,
        )]
        interface: String,

        #[arg(
            long = "enable-egress",
            help = "Enable egress traffic monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        enable_egress: bool,

        #[arg(
            long = "enable-ingress",
            help = "Enable ingress traffic monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        enable_ingress: bool,

        #[arg(
            long = "enable-udp",
            help = "Enable udp protocol network monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        enable_udp: bool,

        #[arg(
            long = "enable-icmp",
            help = "Enable icmp protocol network monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        enable_icmp: bool,

        #[arg(
            long = "enable-tcp",
            help = "Enable tcp protocol network monitoring. [default: true]",
            default_value_t = true,
            required = false
        )]
        enable_tcp: bool,

        #[arg(
            long = "log-interval",
            help = "Enable tcp protocol network monitoring.",
            default_value_t = 5,
            required = false
        )]
        log_interval: u8,

        #[arg(
            long = "full-log",
            help = "Log every packet. With this parameter log-interval don't make sense. [default: false]",
            default_value_t = false,
            required = false
        )]
        full_log: bool,
    },
}
