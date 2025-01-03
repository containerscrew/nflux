#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod egress;
mod egress_vpn;
mod maps;
mod firewall;

use aya_ebpf::bindings::xdp_action::XDP_ABORTED;
use aya_ebpf::{
    macros::xdp,
    programs::XdpContext,
};
use firewall::start_firewall;
use core::mem;
use aya_ebpf::bindings::TC_ACT_SHOT;
use aya_ebpf::macros::classifier;
use aya_ebpf::programs::TcContext;
use crate::egress::try_tc_egress;

// Start xdp firewall if enabled. Attach this program to the physical interface
#[xdp]
pub fn xdp_firewall(ctx: XdpContext) -> u32 {
    match start_firewall(ctx) {
        Ok(ret) => ret,
        Err(_) => XDP_ABORTED,
    }
}

// Start traffic control egress if enabled.
#[classifier]
pub fn tc_egress(ctx: TcContext) -> i32 {
    try_tc_egress(ctx).unwrap_or_else(|_| TC_ACT_SHOT)
}
#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
