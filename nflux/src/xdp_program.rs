use aya::{Ebpf, maps::RingBuf};
use tracing::{debug, error, warn};

use crate::{
    network_event::{process_arp_events, process_networking_event},
    utils::wait_for_shutdown,
};

pub fn attach_xdp_program(
    bpf: &mut Ebpf,
    interface: &String,
) -> anyhow::Result<()> {
    let program: &mut aya::programs::Xdp = bpf.program_mut("xdp_program").unwrap().try_into()?;
    program.load()?;

    if let Err(e) = program.attach(interface, aya::programs::XdpFlags::default()) {
        return Err(anyhow::anyhow!(e));
    }

    debug!("XDP program attached to interface {}", interface);

    Ok(())
}

pub async fn start_xdp_program(
    ebpf: &mut Ebpf,
    log_format: String,
) -> anyhow::Result<()> {
    let xdp_event_ring_map = ebpf
        .take_map("NETWORK_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer NETWORK_EVENT map"))?;
    let ring_buf_net = RingBuf::try_from(xdp_event_ring_map)?;

    let xdp_task = tokio::spawn(async move {
        if let Err(e) = process_networking_event(ring_buf_net, log_format, None).await {
            error!("process_networking_event failed: {:?}", e);
        }
    });

    let arp_event_ring_map = ebpf
        .take_map("ARP_EVENTS")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer ARP_EVENTS map"))?;
    let ring_buf_arp = RingBuf::try_from(arp_event_ring_map)?;

    let arp_task = tokio::spawn(async move {
        if let Err(e) = process_arp_events(ring_buf_arp).await {
            error!("process_arp_events failed: {:?}", e);
        }
    });

    tokio::select! {
        _ = wait_for_shutdown() => {
            warn!("You pressed Ctrl-C, shutting down nflux...");
        }
        _ = xdp_task => {
            warn!("XDP task ended");
        }
        _ = arp_task => {
            warn!("ARP_EVENTS task ended");
        }
    }

    Ok(())
}
