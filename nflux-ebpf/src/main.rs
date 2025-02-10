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
    try_tc(ctx, 1).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[classifier]
pub fn tc_ingress(ctx: TcContext) -> i32 {
    try_tc(ctx, 0).unwrap_or_else(|_| TC_ACT_SHOT)
}

// Start traffic control egress for virtual interface if enabled.
// THIS IS NOT WORKING YET
// #[classifier]
// pub fn tc_egress_virtual(ctx: TcContext) -> i32 {
//     try_tc_virtual(ctx, 1).unwrap_or_else(|_| TC_ACT_SHOT)
// }

// #[classifier]
// pub fn tc_ingress_virtual(ctx: TcContext) -> i32 {
//     try_tc_virtual(ctx, 0).unwrap_or_else(|_| TC_ACT_SHOT)
// }

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
