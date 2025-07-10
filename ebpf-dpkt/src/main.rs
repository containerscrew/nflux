#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod maps;

mod dropped_packets;

mod vmlinux;

use aya_ebpf::{macros::tracepoint, programs::TracePointContext};

use crate::dropped_packets::try_dropped_packets;

#[tracepoint]
pub fn dropped_packets(ctx: TracePointContext) -> u32 {
    // This function is called when a packet is dropped
    match try_dropped_packets(ctx) {
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
