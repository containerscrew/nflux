use std::process::{self, exit};

use aya::{include_bytes_aligned, maps::RingBuf, Ebpf};
use custom_logger::setup_logger;
use libc::getuid;
use nflux_common::Configmap;
use tc_event::process_event;
use tracing::{error, info};
use try_tc::try_traffic_control;
use utils::{is_true, set_mem_limit, wait_for_shutdown};

use crate::utils::check_is_root_user;

mod cli;
mod custom_logger;
mod tc_event;
mod try_tc;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start CLI
    match cli::start_cli() {
        Ok(_) => {
        } // Do nothing
        Err(err) => {
            error!("Error starting the cli {}", err)
        }
    }

    // // Load eBPF program
    // let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    // // Traffic control configuration. This data will be used in a shared ebpf map
    // let configmap = Configmap {
    //     disable_private_ips: 1, // Not implemented yet
    //     enable_udp: is_true(cli.enable_udp),                   // 0 = no, 1 = yes
    //     enable_icmp: is_true(cli.enable_icmp),
    //     enable_tcp: is_true(cli.enable_tcp),
    // };

    // // Attach TC program to the interface
    // try_traffic_control(
    //     &mut bpf,
    //     cli.interface,
    //     cli.enable_ingress,
    //     cli.enable_egress,
    //     configmap,
    // )?;

    // // Traffic control event ring buffer
    // let tc_event_ring_map = bpf
    //     .take_map("TC_EVENT")
    //     .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer TC_EVENT map"))?;

    // let ring_buf = RingBuf::try_from(tc_event_ring_map)?;

    // // Spawn a task to process events
    // tokio::spawn(async move { process_event(ring_buf).await });

    // // Wait for shutdown
    // let _ = wait_for_shutdown().await;

    Ok(())
}
