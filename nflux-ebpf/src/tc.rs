use core::mem;

use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::{
    eth::{EthHdr, EtherType},
    icmp::IcmpHdr,
    ip::{IpProto, Ipv4Hdr, Ipv6Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::TcConfig;

use crate::{
    handlers::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet},
    maps::TC_CONFIG,
};

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

pub fn try_tc(ctx: TcContext, direction: u8) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let tc_config = TC_CONFIG.get(0).ok_or(())?;

    match ethhdr.ether_type {
        EtherType::Ipv4 => {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            handle_ipv4_packet(&ctx, direction, tc_config, ipv4hdr, true)
        }
        EtherType::Ipv6 => {
            let ipv6hdr: Ipv6Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;

            match ipv6hdr.next_hdr {
                IpProto::Tcp => {
                    let _tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
                }
                IpProto::Udp => {
                    let _udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
                }
                IpProto::Icmp => {
                    let _icmphdr: *const IcmpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
                }
                _ => {}
            }

            Ok(TC_ACT_PIPE)
        }
        _ => {
            // Ipv4 under a tunnel? (wireguard)
            let ipv4hdr: Option<Ipv4Hdr> = ctx.load(0).ok();
            if let Some(ipv4hdr) = ipv4hdr {
                handle_ipv4_packet(&ctx, direction, tc_config, ipv4hdr, false)
            } else {
                // Is ipv6? Not implemented :(
                Ok(TC_ACT_PIPE)
            }
        }
    }
}

fn handle_ipv4_packet(
    ctx: &TcContext,
    direction: u8,
    configmap: &TcConfig,
    ipv4hdr: Ipv4Hdr,
    is_ether: bool,
) -> Result<i32, ()> {
    let source = u32::from_be(ipv4hdr.src_addr);
    let destination = u32::from_be(ipv4hdr.dst_addr);
    let total_len = u16::from_be(ipv4hdr.tot_len);
    let ttl = u8::from_be(ipv4hdr.ttl);

    match ipv4hdr.proto {
        IpProto::Tcp => handle_tcp_packet(
            ctx,
            source,
            destination,
            total_len,
            ttl,
            direction,
            is_ether,
        ),
        IpProto::Udp => {
            if configmap.enable_udp == 1 {
                handle_udp_packet(ctx, source, destination, direction, is_ether)
            } else {
                // UDP traffic monitoring is disabled
                Ok(TC_ACT_PIPE)
            }
        }
        IpProto::Icmp => handle_icmp_packet(source, destination, direction),
        _ => Ok(TC_ACT_PIPE),
    }
}
