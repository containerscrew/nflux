use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::{
    arp::ArpHdr,
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, Ipv6Hdr},
};
use nflux_common::dto::{ArpEvent, IpFamily};

use crate::{dto::IpHeader, handle_packet::handle_packet, maps::ARP_EVENTS};

pub fn try_tc(
    ctx: TcContext,
    direction: u8,
) -> Result<i32, ()> {
    // Try to parse Ethernet header from offset 0 (standard L2 packet)
    // Ethernet header is 14 bytes (6 dst MAC + 6 src MAC + 2 EtherType)
    if let Ok(ethhdr) = ctx.load::<EthHdr>(0) {
        // let src_mac = ethhdr.src_addr;
        // let dst_mac = ethhdr.dst_addr;

        match ethhdr.ether_type {
            EtherType::Ipv4 => {
                // Parse IPv4 header, which starts right after Ethernet (offset 14)
                let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;

                let _ = handle_packet(&ctx, direction, IpHeader::V4(ipv4hdr), true);

                return Ok(TC_ACT_PIPE);
            }

            EtherType::Ipv6 => {
                // Parse IPv6 header (starts at same place: offset 14)
                let ipv6hdr: Ipv6Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;

                let _ = handle_packet(&ctx, direction, IpHeader::V6(ipv6hdr), true);

                return Ok(TC_ACT_PIPE);
            }

            EtherType::Arp => {
                let arp_hdr: ArpHdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
                let op_code = u16::from_be(arp_hdr.oper);

                let ip_family = match u16::from_be(arp_hdr.ptype) {
                    0x0800 => IpFamily::Ipv4, // AF_INET
                    0x86DD => IpFamily::Ipv6, // AF_INET6
                    _ => IpFamily::Unknown,
                };

                let event = ArpEvent {
                    op_code,
                    ip_family,
                    sha: arp_hdr.sha,
                    spa: {
                        let mut ip = [0u8; 16];
                        ip[12..16].copy_from_slice(&arp_hdr.spa);
                        ip
                    },
                    tha: arp_hdr.tha,
                    tpa: {
                        let mut ip = [0u8; 16];
                        ip[12..16].copy_from_slice(&arp_hdr.tpa);
                        ip
                    },
                };

                if let Some(mut slot) = ARP_EVENTS.reserve(0) {
                    slot.write(event);
                    slot.submit(0);
                }

                return Ok(TC_ACT_PIPE);
            }

            // If not ether type, try to decode as raw IP (tunnels or VPNs)
            _ => {
                if let Ok(ipv4hdr) = ctx.load::<Ipv4Hdr>(0) {
                    let _ = handle_packet(&ctx, direction, IpHeader::V4(ipv4hdr), false);

                    return Ok(TC_ACT_PIPE);
                } else if let Ok(ipv6hdr) = ctx.load::<Ipv6Hdr>(0) {
                    let _ = handle_packet(&ctx, direction, IpHeader::V6(ipv6hdr), true);

                    return Ok(TC_ACT_PIPE);
                }

                return Ok(TC_ACT_PIPE);
            }
        }
    }

    Ok(TC_ACT_PIPE)
}
