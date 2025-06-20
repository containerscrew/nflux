use std::fmt;

use clap::{value_parser, Parser, Subcommand};
use colored::Colorize;

use crate::utils::set_default_iface;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = print_about(),
    arg_required_else_help = false,
    after_help = print_after_help_message(),
)]
pub struct NfluxCliArgs {
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
        long = "listen-port",
        help = "Filter which port do you want to sniff. Affects both, src and dest ports",
        value_parser = value_parser!(u16).range(1..=65535),
        required = false
    )]
    pub listen_port: Option<u16>,

    #[arg(
        long = "exclude-ports",
        help = "Exclude ports from the logger. For example, do not show logs for port 80 in the log. Affects both, src and dest ports.",
        value_parser = value_parser!(u16).range(1..=65535),
        value_delimiter = ',',
        num_args = 1..,
        required = false
    )]
    pub exclude_port: Option<Vec<u16>>,

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
        help = "How often same ip,port,pid,protocol will be logged. This flag is not used if you don't set --disable-full-log.",
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
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
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

// impl NfluxCliArgs {
//     pub fn init() -> Result<Self> {
//         let args = NfluxCliArgs::parse();
//         args.validate()?;
//         Ok(args)
//     }

//     fn validate(&self) -> Result<(), anyhow::Error> {
//         if self.listen_ports.len() > 16 {
//             return Err(anyhow!(
//                 "Too many ports: {} provided, but max is 16",
//                 self.listen_ports.len()
//             ));
//         }
//         Ok(())
//     }
// }

fn print_about() -> String {
    "eBPF network monitoring tool 🐝".red().italic().to_string()
}

fn print_after_help_message() -> String {
    format!("Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: GPL 3\nIssues: github.com/containerscrew/nflux/issues")
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Sniff dropped packets using tracepoint/skb/kfree_skb
    PktDropped {},
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
        assert_eq!(print_after_help_message(), "Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: GPL 3\nIssues: github.com/containerscrew/nflux/issues")
    }
}
