use core::mem;

use aya_ebpf::{
    bindings::TC_ACT_PIPE,
    programs::TcContext,
};
use network_types::{
    eth::{
        EthHdr,
        EtherType,
    },
    ip::{
        Ipv4Hdr,
        Ipv6Hdr,
    },
};

use crate::handle_packet::{
    handle_packet,
    IpHeader,
};

#[inline]
fn ptr_at<T>(
    ctx: &TcContext,
    offset: usize,
) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

/// try_tc is called from the eBPF program to process packets
/// It tries to parse the packet as an Ethernet frame first, then as a raw IP packet (if using
/// tunnels or VPNs) By default the packet is allowed to pass through (TC_ACT_PIPE)
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
                // let _: ArpHdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
                // Handle ARP packet, which is typically used for address resolution

                // Default case: just let the packet pass through
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
