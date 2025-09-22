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
use network_types::eth::{EthHdr, EtherType};

#[xdp]
pub fn xdp_hello(ctx: XdpContext) -> u32 {
    match unsafe { try_xdp_hello(ctx) } {
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

unsafe fn try_xdp_hello(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    match (*ethhdr).ether_type() {
        Ok(EtherType::Ipv4) => info!(&ctx, "IPv4 packet"),
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
