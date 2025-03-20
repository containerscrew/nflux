use aya_ebpf::{bindings::TC_ACT_PIPE, helpers::bpf_get_current_pid_tgid, programs::TcContext};
use network_types::ip::{IpProto, Ipv4Hdr, Ipv6Hdr};
use nflux_common::TcConfig;

use crate::handle_protocols::{handle_icmp_packet, handle_tcp_packet, handle_udp_packet};

pub enum IpHeader {
    V4(Ipv4Hdr),
    V6(Ipv6Hdr),
}

pub fn handle_packet(
    ctx: &TcContext,
    direction: u8,
    configmap: &TcConfig,
    header: IpHeader,
    is_ether: bool,
) -> Result<i32, ()> {
    match header {
        IpHeader::V4(ipv4hdr) => {
            let source = u32::from_be(ipv4hdr.src_addr);
            let destination = u32::from_be(ipv4hdr.dst_addr);
            let total_len = u16::from_be(ipv4hdr.tot_len);
            let ttl = u8::from_be(ipv4hdr.ttl);

            let tgid = bpf_get_current_pid_tgid();
            let pid = (tgid >> 32) as u32;

            match ipv4hdr.proto {
                IpProto::Tcp => handle_tcp_packet(
                    ctx,
                    source,
                    destination,
                    total_len,
                    ttl,
                    direction,
                    is_ether,
                    "ipv4",
                    pid,
                ),
                IpProto::Udp => {
                    if configmap.enable_udp == 1 {
                        handle_udp_packet(
                            ctx,
                            source,
                            destination,
                            direction,
                            is_ether,
                            "ipv4",
                            pid,
                        )
                    } else {
                        // UDP traffic monitoring is disabled
                        Ok(TC_ACT_PIPE)
                    }
                }
                IpProto::Icmp => handle_icmp_packet(source, destination, direction, pid),
                _ => Ok(TC_ACT_PIPE),
            }
        }
        IpHeader::V6(ipv6hdr) => {
            let proto = ipv6hdr.next_hdr;
            match proto {
                IpProto::Tcp => {
                    // let _tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
                    return Ok(TC_ACT_PIPE);
                }
                IpProto::Udp => {
                    // let _udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
                    return Ok(TC_ACT_PIPE);
                }
                IpProto::Icmp => {
                    // let _icmphdr: *const IcmpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
                    return Ok(TC_ACT_PIPE);
                }
                _ => {}
            }
            Ok(TC_ACT_PIPE)
        }
    }
}
