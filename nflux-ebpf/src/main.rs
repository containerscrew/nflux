#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod handlers;
mod logger;
mod maps;
pub mod tc;

use aya_ebpf::{bindings::TC_ACT_SHOT, macros::classifier, programs::TcContext};
use tc::try_tc;

#[classifier]
pub fn tc_egress(ctx: TcContext) -> i32 {
    // Pass the ctx and 1 which is the direction of the traffic (1: egress, 0: ingress)
    try_tc(ctx, 1).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[classifier]
pub fn tc_ingress(ctx: TcContext) -> i32 {
    // Pass the ctx and 1 which is the direction of the traffic (1: egress, 0: ingress)
    try_tc(ctx, 0).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
