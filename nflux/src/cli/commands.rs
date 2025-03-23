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
            long = "disable-egress",
            help = "Disable egress traffic monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_egress: bool,

        #[arg(
            long = "disable-ingress",
            help = "Disable ingress traffic monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_ingress: bool,

        #[arg(
            long = "disable-udp",
            help = "Disable udp protocol network monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_udp: bool,

        #[arg(
            long = "disable-icmp",
            help = "Disable icmp protocol network monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_icmp: bool,

        #[arg(
            long = "disable-tcp",
            help = "Disable tcp protocol network monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_tcp: bool,

        #[arg(
            long = "log-interval",
            help = "Enable tcp protocol network monitoring. This flag is not used if you don't set --disable-full-log. [default: 5(seconds)]",
            default_value_t = 5,
            required = false
        )]
        log_interval: u8,

        #[arg(
            long = "disable-full-log",
            help = "Disable log for every packet. Then use the flag --log-interval Xs. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_full_log: bool,
    },
    Tlstrace {
        /// Sniffing TLS/SSL traffic using uprobes/uretprobes. Supports openssl and nss
        #[arg(
            long = "openssl-path",
            help = "Path to libssl path",
            default_value = "/lib64/libssl.so.3",
            required = false,
        )]
        openssl_path: String,
    }
}
