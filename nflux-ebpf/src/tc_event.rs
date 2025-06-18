use aya_ebpf::helpers::r#gen::bpf_ktime_get_ns;
use nflux_common::{Configmap, TcEvent};

use crate::maps::{ActiveConnectionKey, ACTIVE_CONNECTIONS, TC_EVENT};

#[inline]
pub unsafe fn log_connection(
    event: &TcEvent,
    configmap: Configmap,
) {
    if configmap.disable_full_log == 0 {
        if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
            unsafe { *data.as_mut_ptr() = *event }
            data.submit(0);
        }
    } else {
        // Get current time
        let current_time = bpf_ktime_get_ns();

        let key = ActiveConnectionKey {
            protocol: event.protocol,
            src_port: event.src_port,
            dst_port: event.dst_port,
            src_ip: event.src_ip,
            dst_ip: event.dst_ip,
        };

        // If the connection (src_port, dst_ip) is already tracked, return
        if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
            // Check if the timestamp is less than 10 seconds
            if current_time - *last_log_time < configmap.log_interval {
                return;
            }
        }

        // Log the connection event
        if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
            unsafe { *data.as_mut_ptr() = *event }
            data.submit(0);
        }

        // Store the active connection: (PID, Destination IP) -> 1 (dummy value)
        ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();
    }
}
