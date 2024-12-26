use core::mem;

use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::programs::TcContext;
use aya_log_ebpf::info;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr, Ipv6Hdr};
use network_types::tcp::TcpHdr;
use network_types::udp::UdpHdr;
use nflux_common::EgressEvent;

use crate::maps::{ACTIVE_CONNECTIONS, EGRESS_EVENT};


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


pub fn try_tc_egress(ctx: TcContext) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    
    match ethhdr.ether_type {
        EtherType::Ipv4 => unsafe {
            info!(&ctx, "is an ipv4 packet!");
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            let destination = u32::from_be(ipv4hdr.dst_addr);

            match ipv4hdr.proto {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                    let dst_port = u16::from_be((*tcphdr).dest);
                    
                    // Check if this destination is already active
                    if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                        // Log only new connections
                        let event = EgressEvent { dst_ip: destination, dst_port: dst_port };
                        EGRESS_EVENT.output(&ctx, &event, 0);

                        // Mark connection as active
                        if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
                            return Err(());
                        }
                    }
                    return Ok(TC_ACT_PIPE)
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                    let dst_port = u16::from_be((*udphdr).dest);
                    let event = EgressEvent { dst_ip: destination, dst_port: dst_port };
                    EGRESS_EVENT.output(&ctx, &event, 0);

                    return Ok(TC_ACT_PIPE)
                }
                _ => {}
            }
        }
        EtherType::Ipv6 => {
            info!(&ctx, "is an ipv6 packet!");
            //let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            //let proto = unsafe { (*ipv6hdr).next_hdr };
            //let destination = unsafe { (*ipv6hdr).dst_addr.in6_u.u6_addr8 };

            // Create here a fake event
            let event = EgressEvent { dst_ip: u32::from_be_bytes([192, 67, 4, 2]), dst_port: 99 };
            EGRESS_EVENT.output(&ctx, &event, 0);
            return Ok(TC_ACT_PIPE)
        }
        EtherType::FibreChannel => {
            info!(&ctx, "ether type fibrechannel!!");
            return Ok(TC_ACT_PIPE)
        }
        EtherType::Arp => {
            info!(&ctx, "ARP!!");
            return Ok(TC_ACT_PIPE)
        }
        _ => {
            //info!(&ctx, "other ether type!");
            return Ok(TC_ACT_PIPE)
        },
    }

    Ok(TC_ACT_PIPE)
}
