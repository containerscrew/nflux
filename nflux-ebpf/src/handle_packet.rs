use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::ip::{IpProto, Ipv4Hdr, Ipv6Hdr};
use nflux_common::Configmap;

use crate::handle_protocols::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet};

pub enum IpHeader {
    V4(Ipv4Hdr),
    V6(Ipv6Hdr),
}

struct ProtocolData {
    src_ip: u32,
    dst_ip: u32,
    total_len: u16,
    ttl: u8,
    proto: IpProto,
}

pub fn handle_packet(
    ctx: &TcContext,
    direction: u8,
    configmap: &Configmap,
    header: IpHeader,
    is_ether: bool,
) -> Result<i32, ()> {
    match header {
        IpHeader::V4(ipv4hdr) => {
            let source = u32::from_be_bytes(ipv4hdr.src_addr);
            let destination = u32::from_be_bytes(ipv4hdr.dst_addr);
            let total_len = u16::from_be_bytes(ipv4hdr.tot_len);
            let ttl = u8::from_be(ipv4hdr.ttl);
            let proto = ipv4hdr.proto;

            let proto_data = ProtocolData {
                src_ip: source,
                dst_ip: destination,
                total_len,
                ttl,
                proto,
            };

            match ipv4hdr.proto {
                IpProto::Tcp => {
                    if configmap.disable_tcp == 0 {
                        handle_tcp_packet(
                            ctx,
                            source,
                            destination,
                            total_len,
                            ttl,
                            direction,
                            is_ether,
                            "ipv4",
                            configmap.log_interval,
                            configmap.disable_full_log,
                        )
                    } else {
                        Ok(TC_ACT_PIPE)
                    }
                }
                IpProto::Udp => {
                    if configmap.disable_udp == 0 {
                        handle_udp_packet(
                            ctx,
                            source,
                            destination,
                            direction,
                            is_ether,
                            "ipv4",
                            configmap.log_interval,
                            configmap.disable_full_log,
                        )
                    } else {
                        // UDP traffic monitoring is disabled
                        Ok(TC_ACT_PIPE)
                    }
                }
                IpProto::Icmp => {
                    if configmap.disable_icmp == 0 {
                        handle_icmp_packet(
                            source,
                            destination,
                            direction,
                            configmap.log_interval,
                            configmap.disable_full_log,
                        )
                    } else {
                        Ok(TC_ACT_PIPE)
                    }
                }
                _ => Ok(TC_ACT_PIPE),
            }
        }
        IpHeader::V6(_ipv6hdr) => {
            // let proto = ipv6hdr.next_hdr;
            // match proto {
            //     IpProto::Tcp => {
            //         // let _tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
            //         return Ok(TC_ACT_PIPE);
            //     }
            //     IpProto::Udp => {
            //         // let _udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
            //         return Ok(TC_ACT_PIPE);
            //     }
            //     IpProto::Icmp => {
            //         // let _icmphdr: *const IcmpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
            //         return Ok(TC_ACT_PIPE);
            //     }
            //     _ => {}
            // }
            Ok(TC_ACT_PIPE)
        }
    }
}
