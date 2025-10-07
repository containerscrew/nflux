use clap::{Args, Parser, Subcommand};
use colored::Colorize;

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
    pub command: Commands,
    #[arg(
        short = 'l',
        long = "log-level",
        global = true,
        help = "Log level for logging tracing. Possible values: info, warn, trace, debug, error.",
        value_parser = ["info", "warn", "trace", "debug", "error"],
        default_value = "info",
        required = false
    )]
    pub log_level: String,

    #[arg(
        long = "log-format",
        global = true,
        help = "Log format for logging tracing. Possible values: text, json.",
        default_value = "text",
        value_parser = ["text", "json"],
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

#[derive(Debug, Subcommand)]
pub enum Commands {
    // #[clap(about = "Sniff packets in L2/3 using traffic control (TC)")]
    // Tc(CommonArgs),
    #[clap(
        about = "Sniff ingress packets in L2/3 using XDP. Near to the NIC driver less overhead."
    )]
    Xdp(XdpArgs),
}

// Args shared between different subcommands (tc and xdp for now)
#[derive(Debug, Args, Clone, PartialEq, Eq)]
pub struct CommonArgs {
    #[arg(short, long, required = false, default_value = "test")]
    pub my_test_flag: String,
}

// Args for XDP subcommand only
#[derive(Debug, Args, Clone, PartialEq, Eq)]
pub struct XdpArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(
        short = 'i',
        long = "interface",
        help = "Interface to attach the program.",
        required = true
    )]
    pub interface: String,
}

fn print_about() -> String {
    "\neBPF network monitoring tool 🐝"
        .red()
        .italic()
        .to_string()
}

fn print_after_help_message() -> String {
    String::from(
        "Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: MIT or GPL3\nIssues: github.com/containerscrew/nflux/issues",
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_about() {
        assert_eq!(
            print_about(),
            "\neBPF network monitoring tool 🐝"
                .red()
                .italic()
                .to_string()
        );
    }

    #[test]
    fn test_print_help_message() {
        assert_eq!(
            print_after_help_message(),
            "Author: containerscrew \nWebsite: github.com/containerscrew/nflux\nLicense: MIT or GPL3\nIssues: github.com/containerscrew/nflux/issues"
        )
    }
}
