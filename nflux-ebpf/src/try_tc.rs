use core::mem;

use crate::handle_packet::{handle_packet, IpHeader};
use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, Ipv6Hdr},
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

// Offset 0:    Ethernet header (14 bytes)
// Offset 14:   IPv4 header (20 bytes min)
// Offset 34:   TCP/UDP header
// Offset 54+:  Payload (e.g. HTTP, DNS, etc.)

/// This function is called from the eBPF program to process packets
/// It tries to parse the packet as an Ethernet frame first, then as a raw IP packet
/// It returns a result indicating whether the packet should be dropped or allowed to pass
pub fn try_tc(ctx: TcContext, direction: u8) -> Result<i32, ()> {
    // Try to parse Ethernet header from offset 0 (standard L2 packet)
    if let Ok(ethhdr) = ctx.load::<EthHdr>(0) {
        // Ethernet header is 14 bytes (6 dst MAC + 6 src MAC + 2 EtherType)
        // let src_mac = ethhdr.src_addr;
        // let dst_mac = ethhdr.dst_addr;

        // Load runtime config from eBPF map
        // let tc_config = TC_CONFIG.get(0).ok_or(())?;

        // Inspect EtherType to know what protocol comes next
        match ethhdr.ether_type {
            // If EtherType is 0x0800 → IPv4
            EtherType::Ipv4 => {
                // Parse IPv4 header, which starts right after Ethernet (offset 14)
                // Ipv4Hdr gives us access to protocol (TCP/UDP), src/dst IPs, etc.
                let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;

                // Now we can process the packet, marking that it was a regular L2 IPv4 frame (true)
                let _ = handle_packet(&ctx, direction, IpHeader::V4(ipv4hdr), true);

                // Default case: just let the packet pass through
                return Ok(TC_ACT_PIPE);
            }

            // If EtherType is 0x86DD → IPv6
            EtherType::Ipv6 => {
                // STEP 3b: Parse IPv6 header (starts at same place: offset 14)
                let ipv6hdr: Ipv6Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;

                let _ = handle_packet(&ctx, direction, IpHeader::V6(ipv6hdr), true);

                // IPv6 header is always 40 bytes, and has src/dst IPs, next-header, etc.
                return Ok(TC_ACT_PIPE);
            }

            // If it's ARP (EtherType 0x0806), just let it pass
            EtherType::Arp => {
                // let _: ArpHdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
                // Handle ARP packet, which is typically used for address resolution

                return Ok(TC_ACT_PIPE);
            }

            // Unknown EtherType, maybe the real IP header is already at offset 0 (e.g. in a tunnel)
            _ => {
                // Fall through to the logic below to try to decode raw IP
                // 🛑 If Ethernet header is missing or invalid:
                // Some VPNs or virtual interfaces (e.g., tun0, wg0) deliver raw IP packets
                // Try to interpret the packet as starting directly with IPv4 or IPv6

                if let Ok(ipv4hdr) = ctx.load::<Ipv4Hdr>(0) {
                    let _ = handle_packet(&ctx, direction, IpHeader::V4(ipv4hdr), false);

                    return Ok(TC_ACT_PIPE);
                } else if let Ok(ipv6hdr) = ctx.load::<Ipv6Hdr>(0) {
                    // We got a raw IPv6 packet, possibly from a tunnel
                    let _ = handle_packet(&ctx, direction, IpHeader::V6(ipv6hdr), true);

                    return Ok(TC_ACT_PIPE);
                }

                return Ok(TC_ACT_PIPE);
            }
        }
    }

    // If nothing matches, just let the packet go through
    Ok(TC_ACT_PIPE)
}
