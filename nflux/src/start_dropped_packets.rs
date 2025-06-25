use aya::{
    maps::{MapData, RingBuf},
    programs::TracePoint,
    Ebpf,
};
use nflux_common::DroppedPacketEvent;
use tokio::sync::watch;
use tracing::{error, info};

use crate::utils::wait_for_shutdown;

pub async fn start_dropped_packets(ebpf: &mut Ebpf) -> anyhow::Result<()> {
    let program: &mut TracePoint = ebpf.program_mut("dropped_packets").unwrap().try_into()?;
    program.load()?;
    program.attach("skb", "kfree_skb")?;
    wait_for_shutdown().await?;

    let dropped_packets_ring_map = ebpf
        .take_map("DROPPED_PACKETS_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer DROPPED_PACKETS_EVENT map"))?;

    let ring_buf = RingBuf::try_from(dropped_packets_ring_map)?;

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let handle = tokio::spawn(async move {
        if let Err(e) = process_dropped_packets(ring_buf, shutdown_rx).await {
            error!("process_event failed: {:?}", e);
        }
    });

    wait_for_shutdown().await?;

    let _ = shutdown_tx.send(true);

    handle.await?;

    Ok(())
}

pub async fn process_dropped_packets(
    mut ring_buf: RingBuf<MapData>,
    mut shutdown: watch::Receiver<bool>,
) -> Result<(), anyhow::Error> {
    loop {
        if *shutdown.borrow() {
            break;
        }

        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();

            if data.len() == size_of::<DroppedPacketEvent>() {
                let event: &DroppedPacketEvent =
                    unsafe { &*(data.as_ptr() as *const DroppedPacketEvent) };

                info!(
                    "Dropped packet! Proto: {} Reason Code: {} Reason: {:?} PID: {} Human friendly: {:?}",
                    event.protocol,
                    event.reason_code,
                    event.reason,
                    event.pid,
                    event.reason_description,
                );
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    break;
                }
            }
        }
    }

    Ok(())
}
