use std::{fmt};

use crate::utils::set_default_iface;
use clap::Parser;
use colored::Colorize;
use anyhow::anyhow;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = set_about(),
    arg_required_else_help = false,
    after_help = print_help_message(),
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
        long = "listen-ports",
        help = "Filter which ports do you want to sniff.",
        value_delimiter = ',',
        num_args = 0..=16,
        required = false
    )]
    pub listen_ports: Vec<u16>,

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

fn set_about() -> String {
    "eBPF network monitoring tool 🐝".red().italic().to_string()
}

fn print_help_message() -> String {
    format!("Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: GPL 3\nIssues: https://github.com/containerscrew/nflux/issues")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_about() {
        assert_eq!(set_about(), "eBPF network monitoring tool 🐝".blue().to_string());
    }

    #[test]
    fn test_print_help_message() {
        assert_eq!(print_help_message(), "Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: GPL 3\nIssues: https://github.com/containerscrew/nflux/issues")
    }

    #[test]
    fn test_nflux_cli_args_display() {
        let args = NfluxCliArgs {
            log_level: "info".to_string(),
            log_format: "text".to_string(),
            interface: "eth0".to_string(),
            disable_egress: false,
            disable_ingress: false,
            disable_udp: false,
            disable_icmp: false,
            disable_tcp: false,
            log_interval: 5,
            disable_full_log: false,
            with_timer: false,
            filter_ports: todo!(),
        };
        assert_eq!(
            format!("{}", args),
            "log_level: info, log_format: text, interface: eth0, \
             disable_egress: false, disable_ingress: false, disable_udp: false, \
             disable_icmp: false, disable_tcp: false, log_interval: 5, disable_full_log: false"
        );
    }
}