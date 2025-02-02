#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod xdp_firewall;
mod traffic_control;
mod maps;

use aya_ebpf::bindings::xdp_action::XDP_ABORTED;
use aya_ebpf::bindings::TC_ACT_SHOT;
use aya_ebpf::macros::classifier;
use aya_ebpf::programs::TcContext;
use aya_ebpf::{macros::xdp, programs::XdpContext};
use xdp_firewall::firewall::start_firewall;
use traffic_control::egress::try_tc_physical;
use crate::traffic_control::egress::try_tc_virtual;

// Start xdp xdp_firewall if enabled. Attach this program to the physical interface
#[xdp]
pub fn xdp_firewall(ctx: XdpContext) -> u32 {
    match start_firewall(ctx) {
        Ok(ret) => ret,
        Err(_) => XDP_ABORTED,
    }
}

// Start traffic control egress for physical interface if enabled.
#[classifier]
pub fn tc_egress_physical(ctx: TcContext) -> i32 {
    try_tc_physical(ctx, 1).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[classifier]
pub fn tc_ingress_physical(ctx: TcContext) -> i32 {
    try_tc_physical(ctx, 0).unwrap_or_else(|_| TC_ACT_SHOT)
}

// Start traffic control egress for virtual interface if enabled.
// THIS IS NOT WORKING YET
#[classifier]
pub fn tc_egress_virtual(ctx: TcContext) -> i32 {
    try_tc_virtual(ctx, 1).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[classifier]
pub fn tc_ingress_virtual(ctx: TcContext) -> i32 {
    try_tc_virtual(ctx, 0).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
