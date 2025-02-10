use aya_ebpf::{helpers::bpf_ktime_get_ns, programs::TcContext};
use nflux_common::TcEvent;

use crate::maps::{ACTIVE_CONNECTIONS, TC_EVENT};

#[inline]
pub unsafe fn log_connection(
    _ctx: &TcContext,
    source: u32,
    destination: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    direction: u8, // 0: ingress, 1: egress
    pid: u32,
    log_every: u32,
) {
    let current_time = (bpf_ktime_get_ns() / 1_000_000_000) as u32;

    let event = TcEvent {
        src_ip: source,
        dst_ip: destination,
        src_port,
        dst_port,
        protocol,
        direction,
        pid,
    };

    if let Some(&last_seen) = ACTIVE_CONNECTIONS.get(&pid) {
        if current_time - last_seen > log_every {
            let _ = ACTIVE_CONNECTIONS.remove(&destination);
        } else {
            return; // Connection is still active, no need to log
        }
    }

    // Log the connection
    if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
        unsafe { *data.as_mut_ptr() = event }

        data.submit(0);
    }

    // Update the last seen time
    ACTIVE_CONNECTIONS.insert(&pid, &current_time, 0).ok();
}
