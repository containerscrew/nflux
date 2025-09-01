#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod dto;
mod handle_packet;
mod logger;
mod maps;
mod try_cgroups;
mod try_dpkt;
mod try_tc;
mod utils;
mod vmlinux;

use aya_ebpf::{
    bindings::TC_ACT_SHOT,
    macros::{cgroup_skb, classifier, tracepoint},
    programs::{SkBuffContext, TcContext, TracePointContext},
};
use try_tc::try_tc;

use crate::{try_cgroups::try_cgroups_traffic, try_dpkt::try_dropped_packets};

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

#[tracepoint]
pub fn dropped_packets(ctx: TracePointContext) -> u32 {
    // This function is called when a packet is dropped
    match try_dropped_packets(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[cgroup_skb]
pub fn cgroups_traffic_egress(ctx: SkBuffContext) -> i32 {
    match try_cgroups_traffic(ctx, 1) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[cgroup_skb]
pub fn cgroups_traffic_ingress(ctx: SkBuffContext) -> i32 {
    match try_cgroups_traffic(ctx, 0) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
