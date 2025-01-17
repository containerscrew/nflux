use aya_ebpf::programs::TcContext;
use nflux_common::EgressEvent;

use crate::maps::{ACTIVE_CONNECTIONS, EGRESS_EVENT};

#[inline]
pub unsafe fn log_connection(
    ctx: &TcContext,
    log_new_connection: u8,
    destination: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    pid: u64,
) {
    // If log_only_new_connections is enabled
    // Only log connections to different endpoints (ips)
    match log_new_connection {
        0 => {
            // Log all connections
            let event = EgressEvent {
                dst_ip: destination,
                src_port,
                dst_port,
                protocol,
                pid,
            };
            EGRESS_EVENT.output(ctx, &event, 0);
        }
        1 => {
            // Check if this destination is already active
            if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                let event = EgressEvent {
                    dst_ip: destination,
                    src_port,
                    dst_port,
                    protocol,
                    pid,
                };
                EGRESS_EVENT.output(ctx, &event, 0);

                // Mark connection as active
                if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
                    return;
                }
            }
        }
        _ => {}
    }
}
