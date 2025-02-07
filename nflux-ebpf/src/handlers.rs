use core::mem;

use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use aya_ebpf::helpers::bpf_get_current_pid_tgid;
use network_types::{
    eth::EthHdr,
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::TcConfig;
use crate::logger::log_connection;

#[inline]
fn ptr_at<T>(ctx: &TcContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}
pub fn handle_icmp_packet(
    ctx: &TcContext,
    source: u32,
    destination: u32,
    direction: u8,
) -> Result<i32, ()> {

    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32; // Extract PID from PID/TGID. First 32 bits are TGID (Thread Group ID) and last 32 bits are PID


    unsafe {
        log_connection(
            ctx,
            source,
            destination,
            0,
            0,
            IpProto::Icmp as u8,
            direction,
            pid,
        )
    };

    Ok(TC_ACT_PIPE)
}

pub fn handle_tcp_packet(
    ctx: &TcContext,
    source: u32,
    destination: u32,
    direction: u8,
) -> Result<i32, ()> {
    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;

    let src_port = u16::from_be((unsafe { *tcphdr }).source);
    let dst_port = u16::from_be((unsafe { *tcphdr }).dest);
    let protocol = IpProto::Tcp as u8;
    let pid = bpf_get_current_pid_tgid() as u32;

    unsafe {
        log_connection(
            ctx,
            source,
            destination,
            src_port,
            dst_port,
            protocol,
            direction,
            pid,
        );
    }

    Ok(TC_ACT_PIPE)
}

pub fn handle_udp_packet(
    ctx: &TcContext,
    source: u32,
    destination: u32,
    direction: u8,
) -> Result<i32, ()> {
    let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
    let src_port = u16::from_be((unsafe { *udphdr }).source);
    let dst_port = u16::from_be((unsafe { *udphdr }).dest);
    let protocol = IpProto::Udp as u8;
    let pid = bpf_get_current_pid_tgid() as u32;

    unsafe {
        log_connection(
            ctx,
            source,
            destination,
            src_port,
            dst_port,
            protocol,
            direction,
            pid,
        )
    };

    Ok(TC_ACT_PIPE)
}
