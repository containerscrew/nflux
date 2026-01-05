use std::time::Duration;
use aya::{Ebpf, maps::RingBuf};
use tracing::{debug, error, info, warn};
use aya::maps::{HashMap, MapData};
use tokio::time::sleep;
use nflux_common::dto::{ActiveConnectionKey, FlowState};
use crate::{
    network_event::{process_arp_events, process_networking_event},
    utils::wait_for_shutdown,
};

// Constantes de configuración (podrías pasarlas por parámetro también)
const CLEANUP_INTERVAL: Duration = Duration::from_secs(5);
const TCP_TIMEOUT_NS: u64 = 60 * 1_000_000_000;
const UDP_TIMEOUT_NS: u64 = 30 * 1_000_000_000;

pub async fn clean_active_connections(
    mut active_connections: HashMap<MapData, ActiveConnectionKey, FlowState>,
) -> Result<(), anyhow::Error> {
    info!("Starting Active Connections Cleaner task...");

    loop {
        sleep(CLEANUP_INTERVAL).await;
        let now_ns = get_boot_time_ns();
        let mut keys_to_delete = Vec::new();

        // CORRECCIÓN: Iteramos directamente, sin match
        for item in active_connections.iter() {
            match item {
                Ok((key, state)) => {
                    let timeout = if key.protocol == 6 {
                        TCP_TIMEOUT_NS
                    } else {
                        UDP_TIMEOUT_NS
                    };

                    if now_ns > state.last_seen_ns {
                        let inactive_duration = now_ns - state.last_seen_ns;
                        if inactive_duration > timeout {
                            keys_to_delete.push(key);
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading map entry: {}", e);
                    // Si falla la lectura de un item, continuamos con el siguiente
                    continue;
                }
            }
        }

        if !keys_to_delete.is_empty() {
            debug!("Cleaning up {} stale connections", keys_to_delete.len());
            for key in keys_to_delete {
                if let Err(e) = active_connections.remove(&key) {
                    error!("Failed to remove stale connection: {}", e);
                }
            }
        }
    }
}

// Helper para obtener el tiempo compatible con bpf_ktime_get_ns (CLOCK_MONOTONIC)
fn get_boot_time_ns() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
    }
    (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
}

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

    let active_connections_map = ebpf
        .take_map("ACTIVE_CONNECTIONS")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ACTIVE_CONNECTIONS map"))?;

    let active_connections = HashMap::try_from(active_connections_map)?;

    let cleaner_task = tokio::spawn(async move {
        if let Err(e) = clean_active_connections(active_connections).await {
            error!("clean_active_connections failed: {:?}", e);
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
        _ = cleaner_task => {
            warn!("Cleaner task ended unexpectedly");
        }
    }

    Ok(())
}
