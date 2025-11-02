use aya_ebpf::helpers::r#gen::bpf_ktime_get_ns;
use nflux_common::{
    dto::{IpFamily, NetworkEvent, TcpFlags},
    maps::NETWORK_EVENT,
};

use crate::{dto::ActiveConnectionKey, maps::ACTIVE_CONNECTIONS};

#[inline]
pub unsafe fn log_connection_fields(
    src_ip: [u8; 16],
    dst_ip: [u8; 16],
    total_len: u16,
    ttl: u8,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    direction: u8,
    ip_family: IpFamily,
    tcp_flags: Option<TcpFlags>,
    disable_full_log: u8,
    log_interval: u64,
) {
    let key = ActiveConnectionKey {
        protocol,
        src_port,
        dst_port,
        src_ip,
        dst_ip,
    };

    let should_log = if disable_full_log == 0 {
        true
    } else {
        let current_time = bpf_ktime_get_ns();

        if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
            if current_time - *last_log_time < log_interval {
                return;
            }
        }

        ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();
        true
    };

    if should_log {
        if let Some(mut data) = NETWORK_EVENT.reserve::<NetworkEvent>(0) {
            let ptr = data.as_mut_ptr();
            core::ptr::write(
                ptr,
                NetworkEvent {
                    src_ip,
                    dst_ip,
                    total_len,
                    ttl,
                    src_port,
                    dst_port,
                    protocol,
                    direction,
                    ip_family,
                    tcp_flags,
                },
            );
            data.submit(0);
        }
    }
}
