use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    about = "nflux",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Network monitoring and firewall using EBPF, XDP and TC. Powered by Aya-rs",
    arg_required_else_help = false
)]

pub struct Args {
    #[arg(
        short = 'c',
        long = "config-file",
        help = "Path to the configuration file nflux.conf",
        default_value = "/etc/nflux/nflux.conf",
        required = false
    )]
    pub config_file: String,
}
