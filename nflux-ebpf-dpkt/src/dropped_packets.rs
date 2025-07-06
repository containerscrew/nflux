use core::{ffi::c_void, mem::offset_of};

use aya_ebpf::{
    helpers::gen::{bpf_get_current_pid_tgid, bpf_probe_read_kernel},
    programs::TracePointContext,
};
use nflux_common::{
    skb_reason::{reason_description, reason_to_str},
    DroppedPacketEvent,
};

use crate::{maps::DROPPED_PACKETS_EVENT, vmlinux::sock};

const REASON_OFFSET: usize = 36;
const PROTO_OFFSET: usize = 32;

const RX_SK_OFFSET: usize = 24;

fn str_to_bytes<const N: usize>(s: &str) -> [u8; N] {
    let mut buf = [0u8; N];
    let bytes = s.as_bytes();
    let len = bytes.len().min(N);
    buf[..len].copy_from_slice(&bytes[..len]);
    buf
}

#[inline(always)]
fn read_sock_field<T>(
    sock_ptr: *const sock,
    field_offset: usize,
) -> Result<T, u32>
where
    T: Default,
{
    let mut value = T::default();
    let ret = unsafe {
        bpf_probe_read_kernel(
            &mut value as *mut _ as *mut c_void,
            size_of::<T>() as u32,
            (sock_ptr as *const u8).wrapping_add(field_offset) as *const c_void,
        )
    };
    if ret != 0 {
        return Err(0);
    }
    Ok(value)
}

// https://github.com/torvalds/linux/blob/master/include/net/dropreason-core.h
// tracepoint format: sudo cat /sys/kernel/debug/tracing/events/skb/kfree_skb/format
pub fn try_dropped_packets(ctx: TracePointContext) -> Result<u32, u32> {
    let reason_code = unsafe { ctx.read_at::<u32>(REASON_OFFSET).map_err(|_| 1u32)? };

    // Skip reason NOT_SPECIFIED (0) and UNKNOWN_REASON
    if reason_code <= 2 {
        return Ok(0);
    }

    let pid: u32 = unsafe { (bpf_get_current_pid_tgid() >> 32) as u32 };

    // Read the pointer to the socket structure
    let rx_sk = unsafe { ctx.read_at::<*const sock>(RX_SK_OFFSET).map_err(|_| 1u32)? };
    if rx_sk.is_null() {
        return Ok(0);
    }

    // Data
    // TODO: add more data like src/dst IP, ports, etc.
    let protocol = read_sock_field::<u16>(rx_sk, offset_of!(sock, sk_protocol))?;
    let family = read_sock_field::<u16>(rx_sk, offset_of!(sock, __sk_common.skc_family))?;
    let _src_ip =
        read_sock_field::<[u8; 16]>(rx_sk, offset_of!(sock, __sk_common.skc_v6_rcv_saddr))?;

    // info!(
    //     &ctx,
    //     "src ip address: {}.{}.{}.{}", src_ip[12], src_ip[13], src_ip[14], src_ip[15]
    // );

    let reason_description = reason_description(reason_code);
    let reason_str = reason_to_str(reason_code);

    let event = DroppedPacketEvent {
        protocol,
        pid,
        reason_code,
        reason: str_to_bytes::<64>(reason_str),
        reason_description: str_to_bytes::<128>(reason_description),
        family,
    };

    if let Some(mut data) = DROPPED_PACKETS_EVENT.reserve::<DroppedPacketEvent>(0) {
        unsafe { *data.as_mut_ptr() = event }
        data.submit(0);
    }

    Ok(0)
}
