use aya::{Ebpf, maps::RingBuf};
use tracing::{debug, error, warn};

use crate::{events::process_xdp_event, utils::wait_for_shutdown};

pub fn attach_xdp_program(
    bpf: &mut Ebpf,
    interface: &String,
) -> anyhow::Result<()> {
    let program: &mut aya::programs::Xdp = bpf.program_mut("xdp_program").unwrap().try_into()?;
    program.load()?;

    if let Err(e) = program.attach(interface, aya::programs::XdpFlags::default()) {
        error!(
            "Failed to attach XDP program to interface {}: {}. Ensure it is a physical interface.",
            interface, e
        );
    }
    debug!("XDP program attached to interface {}", interface);

    Ok(())
}

pub async fn start_xdp_program(ebpf: &mut Ebpf) -> anyhow::Result<()> {
    let xdp_event_ring_map = ebpf
        .take_map("XDP_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer XDP_EVENT map"))?;
    let ring_buf_net = RingBuf::try_from(xdp_event_ring_map)?;

    let xdp_task = tokio::spawn(async move {
        if let Err(e) = process_xdp_event(ring_buf_net).await {
            error!("process_xdp_events failed: {:?}", e);
        }
    });

    tokio::select! {
        _ = wait_for_shutdown() => {
            warn!("You pressed Ctrl-C, shutting down nflux...");
        }
        _ = xdp_task => {
            warn!("XDP task ended");
        }
    }

    Ok(())
}
