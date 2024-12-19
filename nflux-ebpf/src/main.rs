#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

mod egress;
mod maps;

use aya_ebpf::bindings::xdp_action::{XDP_ABORTED, XDP_DROP, XDP_PASS};
use aya_ebpf::helpers::bpf_ktime_get_ns;
use aya_ebpf::maps::lpm_trie::Key;
use aya_ebpf::{
    macros::xdp,
    programs::XdpContext,
};
use maps::{ACTIVE_CONNECTIONS, CONNECTION_EVENTS, CONNECTION_TRACKER, ICMP_RULE, IPV4_RULES};
use core::mem;
use aya_ebpf::bindings::TC_ACT_SHOT;
use aya_ebpf::macros::classifier;
use aya_log_ebpf::info;
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

fn log_new_connection(ctx: &XdpContext, src_addr: u32, dst_port: u16, protocol: u8, action: u8) {
    let event = ConnectionEvent {
        src_addr,
        dst_port,
        protocol,
        action,
    };

    CONNECTION_EVENTS.output(ctx, &event, 0);
}

fn start_nflux(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => process_ipv4(&ctx),
        EtherType::Arp => {
            //let header: ArpHdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            return Ok(XDP_PASS);
        },
        _ => Ok(XDP_DROP),
    }
}

#[inline]
fn process_ipv4(ctx: &XdpContext) -> Result<u32, ()> {
    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(ctx, EthHdr::LEN)? };
    let source_ip = u32::from_be(unsafe { (*ipv4hdr).src_addr });
    let proto = unsafe { (*ipv4hdr).proto };

    // Iterate through prefix lengths for longest prefix match
    for prefix_len in (1..=32).rev() {
        let key = Key::new(
            prefix_len,
            LpmKeyIpv4 {
                prefix_len,
                ip: source_ip & (u32::MAX << (32 - prefix_len)),
            },
        );

        if let Some(rule) = IPV4_RULES.get(&key) {
            match proto {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
                    let syn = unsafe { (*tcphdr).syn() };
                    let ack = unsafe { (*tcphdr).ack() };
                    let fin = unsafe { (*tcphdr).fin() };
                    let rst = unsafe { (*tcphdr).rst() };

                    let connection_key = ((source_ip as u64) << 32) | (dst_port as u64);

                    // Handle ACK packets (responses to outgoing connections)
                    if ack == 1 {
                        // Allow if part of an active egress connection
                        if let Some(_) = unsafe { ACTIVE_CONNECTIONS.get(&source_ip) } {
                            return Ok(XDP_PASS);
                        }
                        // Allow if part of an active incoming connection
                        if let Some(_) = unsafe { CONNECTION_TRACKER.get(&connection_key) } {
                            return Ok(XDP_PASS);
                        }
                        log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                        return Ok(XDP_DROP);
                    }

                    // Handle SYN-ACK packets (handshake response from server)
                    if syn == 1 && ack == 1 {
                        // Allow if part of an active egress connection
                        if let Some(_) = unsafe { ACTIVE_CONNECTIONS.get(&source_ip) } {
                            let timestamp = unsafe { bpf_ktime_get_ns() };
                            let _ = CONNECTION_TRACKER.insert(&connection_key, &timestamp, 0);
                            return Ok(XDP_PASS);
                        }
                        log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                        return Ok(XDP_DROP);
                    }

                    // Handle new connection attempts (SYN packets)
                    if syn == 1 && ack == 0 {
                        if rule.ports.contains(&dst_port) && rule.action == 1 {
                            let timestamp = unsafe { bpf_ktime_get_ns() };
                            let _ = CONNECTION_TRACKER.insert(&connection_key, &timestamp, 0);
                            log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 1);
                            return Ok(XDP_PASS);
                        } else {
                            log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                            return Ok(XDP_DROP);
                        }
                    }

                    // Handle connection closure packets (FIN or RST)
                    if fin == 1 || rst == 1 {
                        let _ = CONNECTION_TRACKER.remove(&connection_key);
                        return Ok(XDP_PASS);
                    }

                    // Default action: drop packets not matching any known or allowed connections
                    log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                    return Ok(XDP_DROP);
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*udphdr).dest });

                    if rule.ports.contains(&dst_port) && rule.action == 1 {
                        log_new_connection(ctx, source_ip, dst_port, IpProto::Udp as u8, 1);
                        return Ok(XDP_PASS);
                    }

                    return Ok(XDP_PASS);
                }
                IpProto::Icmp => {
                    if let Some(&icmp_ping) = ICMP_RULE.get(0) {
                        if icmp_ping == 1 {
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
    }

    // Default action if no rule matches
    Ok(XDP_DROP)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
