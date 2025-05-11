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
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let tc_config = TC_CONFIG.get(0).ok_or(())?;

    match ethhdr.ether_type {
        EtherType::Ipv4 => {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            handle_packet(&ctx, direction, tc_config, IpHeader::V4(ipv4hdr), true)
        }
        EtherType::Ipv6 => {
            let ipv6hdr: Ipv6Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            handle_packet(&ctx, direction, tc_config, IpHeader::V6(ipv6hdr), true)
        }
        EtherType::Arp => {
            // ARP
            Ok(TC_ACT_PIPE)
        }
        _ => {
            // Not EtherType::Ipv4 or EtherType::Ipv6?
            // Using vpn or other tunneling protocol?
            let ipv4hdr: Option<Ipv4Hdr> = ctx.load(0).ok();
            if let Some(ipv4hdr) = ipv4hdr {
                handle_packet(&ctx, direction, tc_config, IpHeader::V4(ipv4hdr), false)
            } else {
                // Is ipv6? Not implemented :(
                // Return TC_ACT_PIPE to pass the packet to the next classifier
                Ok(TC_ACT_PIPE)
            }
        }
    }
}
