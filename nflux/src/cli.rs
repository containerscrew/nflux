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
}
