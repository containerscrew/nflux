use core::mem;

use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};

use network_types::{
    eth::EthHdr,
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::EgressConfig;

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
    egress_config: &EgressConfig,
    destination: u32,
) -> Result<i32, ()> {

    if egress_config.log_icmp_connections == 1 {
        unsafe {
            log_connection(
                ctx,
                egress_config.log_only_new_connections,
                destination,
                0,
                0,
                IpProto::Icmp as u8,
            )
        };
    }

    Ok(TC_ACT_PIPE)
}

pub fn handle_tcp_packet(
    ctx: &TcContext,
    egress_config: &EgressConfig,
    destination: u32,
) -> Result<i32, ()> {
    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;

    let src_port = u16::from_be((unsafe { *tcphdr }).source);
    let dst_port = u16::from_be((unsafe { *tcphdr }).dest);
    let protocol = IpProto::Tcp as u8;

    if egress_config.log_tcp_connections == 1 {
        unsafe {
            log_connection(
                ctx,
                egress_config.log_only_new_connections,
                destination,
                src_port,
                dst_port,
                protocol,
            )
        };
    }

    Ok(TC_ACT_PIPE)
}

pub fn handle_udp_packet(
    ctx: &TcContext,
    egress_config: &EgressConfig,
    destination: u32,
) -> Result<i32, ()> {
    let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
    let src_port = u16::from_be((unsafe { *udphdr }).source);
    let dst_port = u16::from_be((unsafe { *udphdr }).dest);
    let protocol = IpProto::Udp as u8;

    if egress_config.log_udp_connections == 1 {
        unsafe {
            log_connection(
                ctx,
                egress_config.log_only_new_connections,
                destination,
                src_port,
                dst_port,
                protocol,
            )
        };
    }

    Ok(TC_ACT_PIPE)
}
