use clap::{value_parser, Parser, Subcommand};
use colored::Colorize;

use crate::utils::set_default_iface;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = print_about(),
    arg_required_else_help = true,
    after_help = print_after_help_message(),
)]
pub struct NfluxCliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(
        short = 'l',
        long = "log-level",
        global = true,
        help = "Log level for logging tracing. Possible values: info, warn, trace, debug, error.",
        default_value = "info",
        required = false
    )]
    pub log_level: String,

    #[arg(
        long = "log-format",
        global = true,
        help = "Log format for logging tracing. Possible values: text, json.",
        default_value = "text",
        required = false
    )]
    pub log_format: String,

    #[arg(
        long = "with-timer",
        help = "Add timer to the logs. This will add a timestamp to each log entry. Ej: 2025-06-01 23:02:47",
        default_value_t = false,
        global = true,
        required = false
    )]
    pub with_timer: bool,
}

fn print_about() -> String {
    "eBPF network monitoring tool 🐝".red().italic().to_string()
}

fn print_after_help_message() -> String {
    format!("Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: MIT or GPL3\nIssues: github.com/containerscrew/nflux/issues")
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Sniff packets in L2/3 using traffic control (TC)
    Tc {
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
        long = "listen-port",
        help = "Filter which port do you want to sniff. Affects both, src and dest ports",
        value_parser = value_parser!(u16).range(1..=65535),
        required = false
        )]
        listen_port: Option<u16>,

        #[arg(
        long = "exclude-ports",
        help = "Exclude ports from the logger. For example, do not show logs for port 80 in the log. Affects both, src and dest ports.",
        value_parser = value_parser!(u16).range(1..=65535),
        value_delimiter = ',',
        num_args = 1..,
        required = false
        )]
        exclude_port: Option<Vec<u16>>,

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
            long = "disable-arp",
            help = "Disable arp protocol network monitoring. [default: false]",
            default_value_t = false,
            required = false
        )]
        disable_arp: bool,

        #[arg(
            long = "log-interval",
            help = "How often same ip,port,pid,protocol will be logged. This flag is not used if you don't set --disable-full-log.",
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
    // /// Sniff dropped packets using tracepoint/skb/kfree_skb
    // Dpkt {},
    // /// Sniff container traffic using cgroup skb
    // Cgroups {
    //     #[arg(
    //         short = 'c',
    //         long = "cgroup-path",
    //         help = "Cgroup path",
    //         required = false
    //     )]
    //     cgroup_path: Option<String>,
    //     #[arg(
    //         long = "podman-socket-path",
    //         help = "Podman socket path to use for listing containers",
    //         default_value = "/run/user/1000/podman/podman.sock",
    //         required = false
    //     )]
    //     podman_socket_path: String,
    //     #[arg(
    //         long = "containerd-socket-path",
    //         help = "Containerd socket path to use for listing containers",
    //         default_value = "/run/containerd/containerd.sock",
    //         required = false
    //     )]
    //     containerd_socket_path: String,
    // },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_about() {
        assert_eq!(
            print_about(),
            "eBPF network monitoring tool 🐝".red().italic().to_string()
        );
    }

    #[test]
    fn test_print_help_message() {
        assert_eq!(print_after_help_message(), "Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: MIT or GPL3\nIssues: github.com/containerscrew/nflux/issues")
    }
}
