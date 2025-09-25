#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]
use core::mem;

use aya_ebpf::{
    bindings::xdp_action::{self, XDP_PASS},
    macros::xdp,
    programs::XdpContext,
};
use aya_log_ebpf::info;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::Ipv4Hdr,
};
use nflux_common::dto::NetworkEvent;

use crate::maps::XDP_EVENT;

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

            if let Some(mut data) = XDP_EVENT.reserve::<NetworkEvent>(0) {
                let ptr = data.as_mut_ptr();
                core::ptr::write(
                    ptr,
                    NetworkEvent {
                        src_ip,
                        dst_ip,
                        total_len,
                        ttl,
                        src_port: 0,
                        dst_port: 0,
                        protocol: protocol as u8,
                        direction: 0,
                        ip_family: nflux_common::dto::IpFamily::Ipv4,
                        tcp_flags: None,
                    },
                );
                data.submit(0);
            }
        }
        Ok(EtherType::Ipv6) => info!(&ctx, "IPv6 packet"),
        Ok(EtherType::Arp) => info!(&ctx, "ARP packet"),
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
