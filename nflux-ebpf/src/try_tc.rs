use core::mem;

use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, Ipv6Hdr},
};

use crate::{
    handle_packet::{handle_packet, IpHeader},
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
    // Parse Ethernet Header from the very start of the packet (offset 0)
    // Ethernet header is 14 bytes (6 dst MAC + 6 src MAC + 2 EtherType)
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    // let src_mac = ethhdr.src_addr;
    // let dst_mac = ethhdr.dst_addr;

    // Load runtime config from eBPF map
    let tc_config = TC_CONFIG.get(0).ok_or(())?;

    // Inspect EtherType to know what protocol comes next
    match ethhdr.ether_type {
        // If EtherType is 0x0800 → IPv4
        EtherType::Ipv4 => {
            // Parse IPv4 header, which starts right after Ethernet (offset 14)
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            // Ipv4Hdr gives us access to protocol (TCP/UDP), src/dst IPs, etc.

            // Now we can process the packet, marking that it was a regular L2 IPv4 frame (true)
            handle_packet(&ctx, direction, tc_config, IpHeader::V4(ipv4hdr), true)
        }

        // If EtherType is 0x86DD → IPv6
        EtherType::Ipv6 => {
            // STEP 3b: Parse IPv6 header (starts at same place: offset 14)
            let ipv6hdr: Ipv6Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            // IPv6 header is always 40 bytes, and has src/dst IPs, next-header, etc.

            handle_packet(&ctx, direction, tc_config, IpHeader::V6(ipv6hdr), true)
        }

        // If it's ARP (EtherType 0x0806), just let it pass
        EtherType::Arp => Ok(TC_ACT_PIPE),

        // Unknown EtherType, maybe the real IP header is already at offset 0 (e.g. in a tunnel)
        _ => {
            // Try to interpret the packet as if it *started* directly with an IPv4 header
            // This might happen with certain VPNs or encapsulated traffic
            let ipv4hdr: Option<Ipv4Hdr> = ctx.load(0).ok();
            if let Some(ipv4hdr) = ipv4hdr {
                // Treat this as a special case (false = not from classic Ethernet)
                handle_packet(&ctx, direction, tc_config, IpHeader::V4(ipv4hdr), false)
            } else {
                // Still not understood → pass packet along
                Ok(TC_ACT_PIPE)
            }
        }
    }
}

// Offset 0:    Ethernet header (14 bytes)
// Offset 14:   IPv4 header (20 bytes min)
// Offset 34:   TCP/UDP header
// Offset 54+:  Payload (e.g. HTTP, DNS, etc.)
