use aya::{
    Ebpf,
    maps::{MapData, RingBuf},
};
use nflux_common::dto::NetworkEvent;
use tracing::{debug, error, info, warn};

use crate::{network_event::DisplayNetworkEvent, utils::wait_for_shutdown};

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

pub async fn process_xdp_event(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();
            if data.len() == std::mem::size_of::<NetworkEvent>() {
                let event: &NetworkEvent = unsafe { &*(data.as_ptr() as *const NetworkEvent) };
                info!("{}", DisplayNetworkEvent(*event));
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
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
