use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
};
use nflux_common::TcConfig;

use crate::{
    handlers::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet},
    maps::TC_CONFIG,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Test {
    pub data: u64,
}

pub fn try_tc(ctx: TcContext, direction: u8) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let tc_config =  TC_CONFIG.get(0).ok_or(())?;

    match ethhdr.ether_type {
        EtherType::Ipv4 => {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            handle_ipv4_packet(&ctx, direction, tc_config, ipv4hdr, true)
        }
        EtherType::Ipv6 => {
            // IPV6 traffic is not implemented yet
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

    match ipv4hdr.proto {
        IpProto::Tcp => handle_tcp_packet(
            ctx,
            source,
            destination,
            direction,
            is_ether,
            configmap.log_every,
        ),
        IpProto::Udp => {
            if configmap.enable_udp == 1 {
                handle_udp_packet(
                    ctx,
                    source,
                    destination,
                    direction,
                    is_ether,
                    configmap.log_every,
                )
            } else {
                // UDP traffic monitoring is disabled
                Ok(TC_ACT_PIPE)
            }
        }
        IpProto::Icmp => {
            handle_icmp_packet(ctx, source, destination, direction, configmap.log_every)
        }
        _ => Ok(TC_ACT_PIPE),
    }
}
