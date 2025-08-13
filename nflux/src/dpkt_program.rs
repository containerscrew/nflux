use aya::{maps::RingBuf, programs::TracePoint, Ebpf};
use tracing::{error, warn};

use super::events::process_dp_events;
use crate::utils::wait_for_shutdown;

pub async fn start_dropped_packets(
    ebpf: &mut Ebpf,
    log_format: String,
) -> anyhow::Result<()> {
    let program: &mut TracePoint = ebpf.program_mut("dropped_packets").unwrap().try_into()?;
    program.load()?;
    program.attach("skb", "kfree_skb")?;

    let dropped_packets_ring_map = ebpf
        .take_map("DROPPED_PACKETS_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer DROPPED_PACKETS_EVENT map"))?;

    let ring_buf = RingBuf::try_from(dropped_packets_ring_map)?;

    tokio::select! {
        res = process_dp_events(ring_buf, log_format) => {
            if let Err(e) = res {
                error!("process_dp_events failed: {:?}", e);
            }
        },
        _ = wait_for_shutdown() => {
            warn!("You press Ctrl-C, shutting down nflux...");
        }
    }

    Ok(())
}
