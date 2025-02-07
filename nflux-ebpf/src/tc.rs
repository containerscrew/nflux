use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::programs::TcContext;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use nflux_common::TcConfig;
use crate::handlers::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet};
use crate::maps::TC_CONFIG;

fn handle_ipv4_packet(
    ctx: &TcContext,
    direction: u8,
    configmap: &TcConfig,
) -> Result<i32, ()> {
    let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
    let source = u32::from_be(ipv4hdr.src_addr);
    let destination = u32::from_be(ipv4hdr.dst_addr);

    match ipv4hdr.proto {
        IpProto::Tcp => handle_tcp_packet(ctx,  source, destination, direction, configmap),
        IpProto::Udp => {
            if configmap.disable_udp == 0 {
                handle_udp_packet(ctx, source, destination, direction, configmap)
            } else {
                // UDP traffic monitoring is disabled
                Ok(TC_ACT_PIPE)
            }
        },
        IpProto::Icmp => handle_icmp_packet(ctx, source, destination, direction),
        _ => Ok(TC_ACT_PIPE),
    }
}

pub fn try_tc(ctx: TcContext, direction: u8) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let tc_config = unsafe {TC_CONFIG.get(0).ok_or(())?};

    match ethhdr.ether_type {
        EtherType::Ipv4 => handle_ipv4_packet(&ctx, direction, tc_config),
        EtherType::Ipv6 => {
            // IPV6 traffic is not implemented yet
            Ok(TC_ACT_PIPE)
        }
        _ => {
            //debug!(&ctx, "Probably, not an IP packet. Are you using a tunnel?. This is not implemented yet, sorry :(");
            Ok(TC_ACT_PIPE)
        },
    }
}
