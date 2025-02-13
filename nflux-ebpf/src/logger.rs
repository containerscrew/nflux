use nflux_common::TcEvent;

use crate::maps::{ACTIVE_CONNECTIONS, TC_EVENT};

// Define a struct for the key: (pid, destination IP)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConnectionKey {
    pid: u32,
    dst_ip: u32,
}

#[inline]
pub unsafe fn log_connection(
    source: u32,
    destination: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    direction: u8, // 0: ingress, 1: egress
    pid: u32,
) {
    let event = TcEvent {
        src_ip: source,
        dst_ip: destination,
        src_port,
        dst_port,
        protocol,
        direction,
        pid,
    };

    let key = ConnectionKey {
        pid,
        dst_ip: destination,
    };

    // If the connection (pid, dst_ip) is already tracked, return
    if ACTIVE_CONNECTIONS.get(&key).is_some() {
        return;
    }

    // Log the connection event
    if let Some(mut data) = TC_EVENT.reserve::<TcEvent>(0) {
        unsafe { *data.as_mut_ptr() = event }
        data.submit(0);
    }

    // Store the active connection: (PID, Destination IP) -> 1 (dummy value)
    let value: u8 = 1;
    ACTIVE_CONNECTIONS.insert(&key, &value, 0).ok();
}
