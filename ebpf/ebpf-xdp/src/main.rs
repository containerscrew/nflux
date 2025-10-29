#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]
use core::mem;

use aya_ebpf::{
    bindings::xdp_action::{self, XDP_PASS},
    helpers::r#gen::bpf_ktime_get_ns,
    macros::xdp,
    programs::XdpContext,
};
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
};
use nflux_common::dto::{ActiveConnectionKey, NetworkEvent, TcpFlags};

use crate::maps::{ACTIVE_CONNECTIONS, XDP_EVENT};

mod maps;

#[xdp]
pub fn xdp_program(ctx: XdpContext) -> u32 {
    match unsafe { try_xdp_program(ctx) } {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(
    ctx: &XdpContext,
    offset: usize,
) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

unsafe fn try_xdp_program(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    match (*ethhdr).ether_type() {
        Ok(EtherType::Ipv4) => {
            let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let mut src_ip = [0u8; 16];
            let mut dst_ip = [0u8; 16];

            let r_src_ip = unsafe { (*ipv4hdr).src_addr };
            let r_dst_ip = unsafe { (*ipv4hdr).dst_addr };

            src_ip[12..].copy_from_slice(&r_src_ip);
            dst_ip[12..].copy_from_slice(&r_dst_ip);

            let total_len = u16::from_be_bytes(unsafe { (*ipv4hdr).tot_len });
            let protocol = unsafe { (*ipv4hdr).proto };
            let ttl = unsafe { (*ipv4hdr).ttl };
            let mut src_port: u16 = 0;
            let mut dst_port: u16 = 0;
            let mut tcp_flags: Option<TcpFlags> = None;

            match protocol {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    src_port = u16::from_be_bytes(unsafe { (*tcphdr).source });
                    dst_port = u16::from_be_bytes(unsafe { (*tcphdr).dest });
                    tcp_flags = Some(TcpFlags {
                        syn: ((*tcphdr).syn() != 0) as u8,
                        ack: ((*tcphdr).ack() != 0) as u8,
                        fin: ((*tcphdr).fin() != 0) as u8,
                        rst: ((*tcphdr).rst() != 0) as u8,
                        psh: ((*tcphdr).psh() != 0) as u8,
                        urg: ((*tcphdr).urg() != 0) as u8,
                        ece: ((*tcphdr).ece() != 0) as u8,
                        cwr: ((*tcphdr).cwr() != 0) as u8,
                    });
                }
                IpProto::Udp => return Ok(XDP_PASS),
                IpProto::Icmp => return Ok(XDP_PASS),
                _ => return Ok(XDP_PASS),
            }

            // Check if the active connection is already tracked
            let key = ActiveConnectionKey {
                protocol: protocol as u8,
                src_port,
                dst_port,
                src_ip,
                dst_ip,
            };

            let current_time = bpf_ktime_get_ns();
            let log_interval = 120_000_000_000; // 120 seconds in nanoseconds (too much by the moment)

            if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
                if current_time - *last_log_time < log_interval {
                    return Ok(XDP_PASS);
                }
            }

            ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();

            if let Some(mut data) = XDP_EVENT.reserve::<NetworkEvent>(0) {
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
                        protocol: protocol as u8,
                        direction: 0,
                        ip_family: nflux_common::dto::IpFamily::Ipv4,
                        tcp_flags: tcp_flags,
                    },
                );
                data.submit(0);
            }
        }
        Ok(EtherType::Ipv6) => return Ok(XDP_PASS),
        Ok(EtherType::Arp) => return Ok(XDP_PASS),
        // Err(_) => return Ok(XDP_PASS),
        _ => {
            // Other packet
            return Ok(XDP_PASS);
        }
    }
    Ok(xdp_action::XDP_PASS)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
