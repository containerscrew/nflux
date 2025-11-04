use std::{
    env,
    process::{self, exit},
};

use anyhow::Context;
use aya::{Ebpf, include_bytes_aligned, maps::Array};
use logger::LoggerConfig;
use nflux_common::dto::Configmap;
use tracing::{debug, error, info, warn};
use utils::set_mem_limit;

use crate::{
    config::NfluxConfig,
    logger::init_logger,
    utils::{check_is_root, is_true},
    xdp_program::{attach_xdp_program, start_xdp_program},
};

mod config;
mod logger;
mod network_event;
mod utils;
mod xdp_program;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let config_file = env::var("NFLUX_CONFIG_FILE").unwrap_or_else(|_| "nflux.toml".into());
    let config = NfluxConfig::load(&config_file)?;

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

    // Configmap
    let configmap = Configmap {
        enable_udp: is_true(config.agent.enable_udp), // 0 = no, 1 = yes
        enable_icmp: is_true(config.agent.enable_icmp),
        enable_tcp: is_true(config.agent.enable_tcp),
        enable_arp: is_true(config.agent.enable_arp),
        log_interval: config.agent.log_interval * 1_000_000_000,
        listen_port: config.agent.listen_port.unwrap_or(0), /* Default to 0 if not provided
                                                             * exclude_ports: config.agent.exclude_ports.unwrap().as_slice().try_into()?, */
    };

    info!("Starting nflux with pid {}", process::id());

    match config.agent.mode.as_str() {
        "xdp" => {
            let mut ebpf_xdp =
                Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/xdp")))?;
            attach_xdp_program(&mut ebpf_xdp, &config.agent.interface)?;
            populate_configmap(&mut ebpf_xdp, configmap)?;
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
        "tc" => warn!("Operating in TC mode. Not implemented yet to this new agent version."),
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

fn populate_configmap(
    bpf: &mut Ebpf,
    config: Configmap,
) -> anyhow::Result<(), anyhow::Error> {
    let mut configmap = Array::<_, Configmap>::try_from(
        bpf.map_mut("CONFIGMAP")
            .context("Failed to find CONFIGMAP map")?,
    )?;

    configmap
        .set(0, config, 0)
        .context("Failed to set CONFIGMAP")?;

    debug!("eBPF map CONFIGMAP successfully loaded with struct Configmap");

    Ok(())
}
