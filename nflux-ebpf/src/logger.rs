use aya_ebpf::{programs::TcContext};
use nflux_common::{TcEvent};
use crate::maps::TC_EVENT;

#[inline]
pub unsafe fn log_connection(
    ctx: &TcContext,
    source: u32,
    destination: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    direction: u8, // 0: ingress, 1: egress
    pid: u32,
) {

    let event = TcEvent {
        src_ip: source,
        dst_ip: destination,
        src_port,
        dst_port,
        protocol,
        direction,
        pid
    };

    TC_EVENT.output(ctx, &event, 0);
}
