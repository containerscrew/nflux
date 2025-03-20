use aya_ebpf::helpers::gen::bpf_ktime_get_ns;
use nflux_common::{IpType, TcEvent};
use crate::maps::{ActiveConnectionKey, ACTIVE_CONNECTIONS, TC_EVENT};

#[inline]
pub unsafe fn log_connection(
    source: u32,
    destination: u32,
    total_len: u16,
    ttl: u8,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    direction: u8, // 0: ingress, 1: egress
    proto: &str,   // ipv4 or ipv6
    pid: u32,
) {
    let event = TcEvent {
        src_ip: source,
        dst_ip: destination,
        total_len,
        ttl,
        src_port,
        dst_port,
        protocol,
        direction,
        ip_type: if proto == "ipv4" {
            IpType::Ipv4
        } else {
            IpType::Ipv6
        },
        pid,
    };

    // Get current time
    let current_time = bpf_ktime_get_ns();

    let key = ActiveConnectionKey {
        port:  if direction == 1 { src_port as u32 } else { dst_port as u32},
        ip:  if direction == 1 { destination } else {source},
    };

    // If the connection (src_port, dst_ip) is already tracked, return
    if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
        // Check if the timestamp is less than 10 seconds
        if current_time - *last_log_time < 10_000_000_000 {
            return;
        }
    }

    // Log the connection event
    if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
        unsafe { *data.as_mut_ptr() = event }
        data.submit(0);
    }

    // Store the active connection: (PID, Destination IP) -> 1 (dummy value)
    ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();
}
