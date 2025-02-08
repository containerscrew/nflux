use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::programs::TcContext;
use aya_log_ebpf::info;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use network_types::tcp::TcpHdr;
use nflux_common::TcConfig;
use crate::handlers::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet};
use crate::maps::TC_CONFIG;

pub fn try_tc(ctx: TcContext, direction: u8) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let tc_config = unsafe {TC_CONFIG.get(0).ok_or(())?};

    match ethhdr.ether_type {
        EtherType::Ipv4 => {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            handle_ipv4_packet(&ctx, direction, tc_config, ipv4hdr, true)
        },
        EtherType::Ipv6 => {
            // IPV6 traffic is not implemented yet
            Ok(TC_ACT_PIPE)
        }
        _ => {
            // Ipv4 under a tunnel? (wireguard)
            let ipv4hdr: Option<Ipv4Hdr> = ctx.load(0).ok();
            if let Some(ipv4hdr) = ipv4hdr {
                let destination = u32::from_be(ipv4hdr.dst_addr);
                let source = u32::from_be(ipv4hdr.src_addr);
                handle_ipv4_packet(&ctx, direction, tc_config, ipv4hdr, false)
            } else {
                // Is ipv6? Not implemented :(
                Ok(TC_ACT_PIPE)
            }
        },
    }
}

fn handle_ipv4_packet(
    ctx: &TcContext,
    direction: u8,
    configmap: &TcConfig,
    ipv4hdr: Ipv4Hdr,
    is_ether: bool,
) -> Result<i32, ()> {
    //let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
    let source = u32::from_be(ipv4hdr.src_addr);
    let destination = u32::from_be(ipv4hdr.dst_addr);

    match ipv4hdr.proto {
        IpProto::Tcp => {
            handle_tcp_packet(ctx,  source, destination, direction, is_ether)
        },
        IpProto::Udp => {
            if configmap.enable_udp == 1 {
                handle_udp_packet(ctx, source, destination, direction, is_ether)
            } else {
                // UDP traffic monitoring is disabled
                Ok(TC_ACT_PIPE)
            }
        },
        IpProto::Icmp => handle_icmp_packet(ctx, source, destination, direction),
        _ => Ok(TC_ACT_PIPE),
    }
}

// match ipv4hdr.proto {
//     IpProto::Tcp => {
//         let tcphdr: TcpHdr = ctx.load(20).map_err(|_| ())?;
//         let src_port = u16::from_be(tcphdr.source);
//         let dst_port = u16::from_be(tcphdr.dest);
//         Ok(TC_ACT_PIPE)
//     }
//     IpProto::Udp => {Ok(TC_ACT_PIPE)}
//     IpProto::Icmp => {
//         info!(&ctx, "Wow! ICMP connection under a tunnel. Source: {}, Destination: {}", source, destination);
//         Ok(TC_ACT_PIPE)
//     },
//     _ => {
//         //info!(&ctx, "Probably, ipv6 traffic");
//         Ok(TC_ACT_PIPE)
//     }
// }
