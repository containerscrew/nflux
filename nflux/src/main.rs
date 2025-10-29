use std::process::{self, exit};

use aya::{Ebpf, include_bytes_aligned};
use logger::LoggerConfig;
use tracing::{error, info};
use utils::set_mem_limit;

use crate::{
    config::NfluxConfig,
    logger::init_logger,
    utils::check_is_root,
    xdp_program::{attach_xdp_program, start_xdp_program},
};

mod config;
mod logger;
mod network_event;
mod tc_program;
mod utils;
mod xdp_program;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let config = NfluxConfig::load("nflux.toml")?;

    init_logger(LoggerConfig {
        level: config.logging.log_level,
        format: config.logging.log_type.clone(),
        with_timer: config.logging.with_timer,
    });

    if let Err(e) = check_is_root() {
        error!("{e}");
        exit(1);
    }

    set_mem_limit();

    info!("Starting nflux with pid {}", process::id());

    match config.agent.mode.as_str() {
        "xdp" => {
            let mut ebpf_xdp =
                Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/xdp")))?;
            attach_xdp_program(&mut ebpf_xdp, &config.agent.interface)?;
            info!(
                "Sniffing ingress packets in the NIC {}",
                config.agent.interface
            );
            // Uncomment the following line to enable eBPF logging
            // if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf_xdp) {
            //     warn!("failed to initialize eBPF logger: {e}");
            // }
            start_xdp_program(&mut ebpf_xdp, config.logging.log_type).await?;
        }
        "tc" => info!("Operating in TC mode"),
        other => {
            error!(
                "Invalid mode '{}' in configuration. Use 'xdp' or 'tc'.",
                other
            );
            exit(1);
        }
    }

    Ok(())
}
