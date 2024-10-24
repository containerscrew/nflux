use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    about = "ebpfw",
    version = env!("CARGO_PKG_VERSION"),
    author = "Containerscrew info@containerscrew.com",
    about = "Firewall using EBPF, XDP and TC. Powered by Aya-rs",
    arg_required_else_help = false
)]

pub struct Args {
    #[arg(
        short = 'c',
        long = "config-file",
        help = "Path to the configuration file config.toml",
        default_value = "./config.toml",
        required = false
    )]
    pub config_file: String,
}