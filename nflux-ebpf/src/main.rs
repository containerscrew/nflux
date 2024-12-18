#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod egress;
mod maps;

use aya_ebpf::bindings::xdp_action::{XDP_ABORTED, XDP_DROP, XDP_PASS};
use aya_ebpf::maps::lpm_trie::Key;
use aya_ebpf::{
    macros::xdp,
    programs::XdpContext,
};
use maps::{CONNECTION_EVENTS, ICMP_RULE, IPV4_RULES};
use core::mem;
use aya_ebpf::bindings::TC_ACT_SHOT;
use aya_ebpf::macros::classifier;
use aya_ebpf::programs::TcContext;
use network_types::ip::IpProto;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::Ipv4Hdr,
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{ConnectionEvent, LpmKeyIpv4};
use crate::egress::try_tc_egress;

#[xdp]
pub fn nflux(ctx: XdpContext) -> u32 {
    match start_nflux(ctx) {
        Ok(ret) => ret,
        Err(_) => XDP_ABORTED,
    }
}

#[classifier]
pub fn tc_egress(ctx: TcContext) -> i32 {
    try_tc_egress(ctx).unwrap_or_else(|_| TC_ACT_SHOT)
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn log_new_connection(ctx: XdpContext, src_addr: u32, dst_port: u16, protocol: u8, action: u8) {
    let event = ConnectionEvent {
        src_addr,
        dst_port,
        protocol,
        action,
    };

    CONNECTION_EVENTS.output(&ctx, &event, 0);
}

fn start_nflux(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {
            let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let source_ip = u32::from_be(unsafe { (*ipv4hdr).src_addr });
            let proto = unsafe { (*ipv4hdr).proto };

            // Use LPM map to find the longest prefix match for the source IP
            if let Some(rule) = IPV4_RULES.get(&Key::new(32, LpmKeyIpv4 { prefix_len: 32, ip: source_ip })) {
                match proto {
                    IpProto::Tcp => {
                        let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                        let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
                        let syn = unsafe { (*tcphdr).syn() };
                        let ack = unsafe { (*tcphdr).ack() };

                        if ack == 1 {
                            // Allow established connections
                            return Ok(XDP_PASS);
                        }

                        if syn == 1 {
                            if rule.ports.contains(&dst_port) && rule.action == 1 {
                                log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 1);
                                return Ok(XDP_PASS);
                            } else {
                                log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                                return Ok(XDP_DROP);
                            }
                        }

                        // Default action for unmatched TCP packets
                        return Ok(XDP_DROP);
                    }
                    IpProto::Udp => {
                        let udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                        let dst_port = u16::from_be(unsafe { (*udphdr).dest });

                        // By default UDP packets are always allowed to pass dynamic DNS ports using mDNS and systemd-resolved
                        // By the moment this is enabled to allow user to resolve DNS queries

                        // Do not log anything, not necessary by the moment
                        if rule.ports.contains(&dst_port) && rule.action == 0 {
                            //log_new_connection(ctx, source_ip, dst_port, 17, 1);
                            return Ok(XDP_DROP);
                        }

                        // Allow by default all UDP packets
                        // log_new_connection(ctx, source_ip, dst_port, 17, 0);
                        return Ok(XDP_PASS);
                    }
                    IpProto::Icmp => {
                        if let Some(&icmp_ping) = ICMP_RULE.get(0) {
                            if icmp_ping == 1 {
                                // If ICMP is enabled, allow all ICMP packets
                                log_new_connection(ctx, source_ip, 0, IpProto::Icmp as u8, 1);
                                return Ok(XDP_PASS);
                            }
                        }
                        log_new_connection(ctx, source_ip, 0, IpProto::Icmp as u8, 0);
                        return Ok(XDP_DROP);
                    }
                    _ => return Ok(XDP_DROP),
                }
            }
            Ok(XDP_DROP)
        }
        _ => Ok(XDP_DROP),
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
