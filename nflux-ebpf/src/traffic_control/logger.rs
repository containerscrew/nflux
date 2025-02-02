use aya_ebpf::{helpers::bpf_ktime_get_ns, programs::TcContext};
use nflux_common::EgressEvent;

use crate::maps::{ACTIVE_EGRESS_CONNECTIONS, ACTIVE_INGRESS_CONNECTIONS, EGRESS_EVENT};

#[inline]
pub unsafe fn log_connection(
    ctx: &TcContext,
    log_new_connection: u8,
    log_every: u32,
    source: u32,
    destination: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    direction: u8, // 0: ingress, 1: egress
) {
    let current_time = (bpf_ktime_get_ns() / 1_000_000_000) as u32; // Convert to seconds

    let event = EgressEvent {
        src_ip: source,
        dst_ip: destination,
        src_port,
        dst_port,
        protocol,
        direction,
    };

    match log_new_connection {
        0 => {
            EGRESS_EVENT.output(ctx, &event, 0);
        }
        1 => {
            match direction {
                0 => {
                    if let Some(&last_seen) = ACTIVE_INGRESS_CONNECTIONS.get(&destination) {
                        if current_time - last_seen > 5 {
                            // 60-second timeout
                            let _ = ACTIVE_INGRESS_CONNECTIONS.remove(&destination);
                        } else {
                            return; // Connection is still active, no need to log
                        }
                    }
                    EGRESS_EVENT.output(ctx, &event, 0);
                    ACTIVE_INGRESS_CONNECTIONS
                        .insert(&destination, &current_time, 0)
                        .ok();
                }
                1 => {
                    if let Some(&last_seen) = ACTIVE_EGRESS_CONNECTIONS.get(&destination) {
                        if current_time - last_seen > log_every {
                            let _ = ACTIVE_EGRESS_CONNECTIONS.remove(&destination);
                        } else {
                            return;
                        }
                    }
                    EGRESS_EVENT.output(ctx, &event, 0);
                    ACTIVE_EGRESS_CONNECTIONS
                        .insert(&destination, &current_time, 0)
                        .ok();
                }
                _ => {}
            }
        }
        _ => {}
    }
}
