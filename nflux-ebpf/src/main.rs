#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod handle_packet;
mod maps;
mod tc_event;
mod try_tc;

use aya_ebpf::{
    bindings::TC_ACT_SHOT,
    macros::classifier,
    programs::TcContext,
};
use try_tc::try_tc;

#[classifier]
pub fn tc_egress(ctx: TcContext) -> i32 {
    // Pass the ctx and 1 which is the direction of the traffic (1: egress, 0: ingress)
    // This is used to determine the direction of the traffic
    try_tc(ctx, 1).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[classifier]
pub fn tc_ingress(ctx: TcContext) -> i32 {
    // Pass the ctx and 0 which is the direction of the traffic (1: egress, 0: ingress)
    // This is used to determine the direction of the traffic
    try_tc(ctx, 0).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
