use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::programs::TcContext;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use nflux_common::EgressConfig;

use crate::handlers::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet};
use crate::maps::EGRESS_CONFIG;

fn handle_ipv4_packet(
    ctx: &TcContext,
    egress_config: &EgressConfig,
    direction: u8,
) -> Result<i32, ()> {
    let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
    let source = u32::from_be(ipv4hdr.src_addr);
    let destination = u32::from_be(ipv4hdr.dst_addr);

    match ipv4hdr.proto {
        IpProto::Tcp => handle_tcp_packet(ctx, egress_config, source, destination, direction),
        IpProto::Udp => handle_udp_packet(ctx, egress_config, source, destination, direction),
        IpProto::Icmp => handle_icmp_packet(ctx, egress_config, source, destination, direction),
        _ => Ok(TC_ACT_PIPE),
    }
}

pub fn try_tc_physical(ctx: TcContext, direction: u8) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let egress_config = EGRESS_CONFIG.get(0).ok_or(())?;

    match ethhdr.ether_type {
        EtherType::Ipv4 => handle_ipv4_packet(&ctx, &egress_config, direction),
        EtherType::Ipv6 => {
            // IPV6 traffic is not implemented yet
            Ok(TC_ACT_PIPE)
        }
        _ => Ok(TC_ACT_PIPE),
    }
}

// pub fn try_tc_egress_virtual(ctx: TcContext) -> Result<i32, ()> {
//     let egress_config = EGRESS_CONFIG.get(0).ok_or(())?;

//     // Parse IPv4 o IPv6 header
//     let ipv4hdr: Option<Ipv4Hdr> = ctx.load(0).ok();
//     let ipv6hdr: Option<Ipv6Hdr> = ctx.load(0).ok();

//     if let Some(ipv4hdr) = ipv4hdr {
//         let destination = u32::from_be(ipv4hdr.dst_addr);

//         match ipv4hdr.proto {
//             IpProto::Tcp => handle_tcp_packet(&ctx, egress_config, destination),
//             IpProto::Udp => handle_udp_packet(&ctx, egress_config, destination),
//             IpProto::Icmp => handle_icmp_packet(&ctx, egress_config, destination),
//             _ => {
//                 //info!(&ctx, "Probably, ipv6 traffic");
//                 Ok(TC_ACT_PIPE)
//             }
//         }
//     } else if let Some(_) = ipv6hdr {
//         // IPV6 traffic is not implemented yet

//         // match ipv6hdr.next_hdr {
//         //     IpProto::Tcp => handle_tcp_packet(&ctx, egress_config, u32::from_be_bytes(unsafe {ipv6hdr.dst_addr.in6_u.u6_addr8[0..4].try_into().unwrap()})),
//         //     IpProto::Udp => handle_udp_packet(&ctx, egress_config, u32::from_be_bytes(unsafe { ipv6hdr.dst_addr.in6_u.u6_addr8[0..4].try_into().unwrap() })),
//         //     //IpProto::Icmpv6 => handle_icmpv6_packet(&ctx, egress_config, ipv6hdr.dst_addr),
//         //     _ => Ok(TC_ACT_PIPE),
//         // }
//         info!(&ctx, "Probably, ipv6 traffic");
//         Ok(TC_ACT_PIPE)
//     } else {
//         Ok(TC_ACT_PIPE)
//     }
// }
