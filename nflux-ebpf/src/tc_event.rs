use aya_ebpf::helpers::gen::bpf_ktime_get_ns;
use nflux_common::TcEvent;

use crate::{
    handle_packet::PacketData,
    maps::{ActiveConnectionKey, ACTIVE_CONNECTIONS, TC_EVENT},
};

#[inline]
pub unsafe fn log_connection(
    packet_data: &PacketData,
    src_mac: [u8; 6],
    dst_mac: [u8; 6],
    log_interval: u8,
    disable_full_log: u8,
) {
    let event = TcEvent {
        src_mac,
        dst_mac,
        src_ip: packet_data.src_ip,
        dst_ip: packet_data.dst_ip,
        total_len: packet_data.total_len,
        ttl: packet_data.ttl,
        src_port: packet_data.src_port,
        dst_port: packet_data.dst_port,
        protocol: packet_data.proto,
        direction: packet_data.direction,
        ip_family: packet_data.ip_family,
    };

    if disable_full_log == 0 {
        if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
            unsafe { *data.as_mut_ptr() = event }
            data.submit(0);
        }
    } else {
        // Get current time
        let current_time = bpf_ktime_get_ns();

        let key = ActiveConnectionKey {
            port: if packet_data.direction == 1 {
                packet_data.src_port as u32
            } else {
                packet_data.dst_port as u32
            },
            ip: if packet_data.direction == 1 {
                packet_data.dst_ip
            } else {
                packet_data.src_ip
            },
        };

        // If the connection (src_port, dst_ip) is already tracked, return
        if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
            // Check if the timestamp is less than 10 seconds
            if current_time - *last_log_time < log_interval as u64 * 1_000_000_000 {
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
}
