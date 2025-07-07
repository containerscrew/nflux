use aya_ebpf::helpers::r#gen::bpf_ktime_get_ns;
use nflux_common::{Configmap, IpFamily, TcEvent, TcpFlags};

use crate::maps::{ActiveConnectionKey, ACTIVE_CONNECTIONS, TC_EVENT};

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
    tcp_flags: TcpFlags,
    configmap: Configmap,
) {
    // Construir ActiveConnectionKey sin TcEvent completo
    let key = ActiveConnectionKey {
        protocol,
        src_port,
        dst_port,
        src_ip,
        dst_ip,
    };

    let should_log = if configmap.disable_full_log == 0 {
        true
    } else {
        let current_time = bpf_ktime_get_ns();

        if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
            if current_time - *last_log_time < configmap.log_interval {
                return;
            }
        }

        ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();
        true
    };

    if should_log {
        if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
            let ptr = data.as_mut_ptr();
            core::ptr::write(
                ptr,
                TcEvent {
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

// #[inline]
// pub unsafe fn log_connection(
//     event: &TcEvent,
//     configmap: Configmap,
// ) {
//     // By default, we log all events
//     if configmap.disable_full_log == 0 {
//         if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
//             unsafe {
//                 core::ptr::write(data.as_mut_ptr(), *event);
//             }
//             data.submit(0);
//         }
//     } else {
//         let current_time = bpf_ktime_get_ns();

//         let key = ActiveConnectionKey {
//             protocol: event.protocol,
//             src_port: event.src_port,
//             dst_port: event.dst_port,
//             src_ip: event.src_ip,
//             dst_ip: event.dst_ip,
//         };

//         if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
//             if current_time - *last_log_time < configmap.log_interval {
//                 return;
//             }
//         }

//         if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
//             unsafe {
//                 core::ptr::write(data.as_mut_ptr(), *event);
//             }
//             data.submit(0);
//         }

//         ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();
//     }
// }

// if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
//     unsafe {
//         let ptr = data.as_mut_ptr();
//         core::ptr::write(ptr, TcEvent { ... });
//     }
//     data.submit(0);
// }
