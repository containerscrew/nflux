#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

use aya_ebpf::maps::lpm_trie::Key;
use aya_ebpf::maps::{Array, LpmTrie};
use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::PerfEventArray,
    programs::XdpContext,
};
use core::mem;
use network_types::ip::{IpProto, Ipv6Hdr};
use network_types::{
    eth::{EthHdr, EtherType},
    ip::Ipv4Hdr,
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{ConnectionEvent, IpRule, LpmKeyIpv4, LpmKeyIpv6};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[map]
static IPV4_RULES: LpmTrie<LpmKeyIpv4, IpRule> = LpmTrie::with_max_entries(1024, 0);

#[map]
static IPV6_RULES: LpmTrie<LpmKeyIpv6, IpRule> = LpmTrie::with_max_entries(1024, 0);

#[map]
static CONNECTION_EVENTS: PerfEventArray<ConnectionEvent> = PerfEventArray::new(0);

#[map]
static ICMP_RULE: Array<u32> = Array::with_max_entries(1, 0);

#[xdp]
pub fn nflux(ctx: XdpContext) -> u32 {
    match start_nflux(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
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
                            let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                            let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
                            let syn = unsafe { (*tcphdr).syn() };
                            let ack = unsafe { (*tcphdr).ack() };

                            // Check if the port is in the allowed list for the rule
                            if !rule.ports.contains(&dst_port) {
                                log_new_connection(ctx, source_ip, dst_port, 6, 0);
                                return Ok(xdp_action::XDP_DROP);
                            }

                            // Handle SYN (new connection attempts)
                            if syn == 1 && ack == 0 {
                                if rule.action == 1 {
                                    // Log only connection establishment attempts
                                    log_new_connection(ctx, source_ip, dst_port, 6, rule.action);
                                    return Ok(xdp_action::XDP_PASS);
                                } else {
                                    log_new_connection(ctx, source_ip, dst_port, 6, 0);
                                    return Ok(xdp_action::XDP_DROP);
                                }
                            }

                            // Handle SYN-ACK (response to outgoing connection attempts)
                            if syn == 1 && ack == 1 {
                                return Ok(xdp_action::XDP_PASS);
                            }

                            // Handle ACK (established connections)
                            if ack == 1 {
                                return Ok(xdp_action::XDP_PASS);
                            }

                            // For other TCP packets, apply the rule's action
                            if rule.action == 1 {
                                log_new_connection(ctx, source_ip, dst_port, 6, 1);
                                return Ok(xdp_action::XDP_PASS);
                            }

                            log_new_connection(ctx, source_ip, dst_port, 6, 0);
                            return Ok(xdp_action::XDP_DROP);
                        }
                        IpProto::Udp => {
                            let udphdr: *const UdpHdr =
                                unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                            let dst_port = u16::from_be(unsafe { (*udphdr).dest });

                            if rule.ports.contains(&dst_port) && rule.action == 1 {
                                log_new_connection(ctx, source_ip, dst_port, 17, 0);
                                return Ok(xdp_action::XDP_PASS);
                            }
                            return Ok(xdp_action::XDP_PASS);
                        }
                        IpProto::Icmp => {
                            if let Some(&icmp_ping) = ICMP_RULE.get(0) {
                                if icmp_ping == 1 {
                                    log_new_connection(ctx, source_ip, 0, 1, 1);
                                    return Ok(xdp_action::XDP_PASS);
                                }
                            }
                            return Ok(xdp_action::XDP_DROP);
                        }
                        _ => return Ok(xdp_action::XDP_DROP),
                    }
                }
            }
            Ok(xdp_action::XDP_DROP)
        }
        EtherType::Ipv6 => {
            let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let proto = unsafe { (*ipv6hdr).next_hdr };
            let source_ip = unsafe { (*ipv6hdr).src_addr.in6_u.u6_addr8 };

            for prefix_len in (1..=128).rev() {
                let key = Key::new(
                    prefix_len,
                    LpmKeyIpv6 {
                        prefix_len,
                        ip: source_ip,
                    },
                );

                if let Some(rule) = IPV6_RULES.get(&key) {
                    match proto {
                        IpProto::Tcp => {
                            let tcphdr: *const TcpHdr =
                                unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)? };
                            let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
                            let syn = unsafe { (*tcphdr).syn() };
                            let ack = unsafe { (*tcphdr).ack() };

                            // Check if the port is in the allowed list for the rule
                            if !rule.ports.contains(&dst_port) {
                                log_new_connection(ctx, 0, dst_port, 6, 0);
                                return Ok(xdp_action::XDP_DROP);
                            }

                            // Handle SYN (new connection attempts)
                            if syn == 1 && ack == 0 {
                                if rule.action == 1 {
                                    log_new_connection(ctx, 0, dst_port, 6, 1);
                                    return Ok(xdp_action::XDP_PASS);
                                } else {
                                    log_new_connection(ctx, 0, dst_port, 6, 0);
                                    return Ok(xdp_action::XDP_DROP);
                                }
                            }

                            // Handle SYN-ACK (response to outgoing connection attempts)
                            if syn == 1 && ack == 1 {
                                log_new_connection(ctx, 0, dst_port, 6, 1);
                                return Ok(xdp_action::XDP_PASS);
                            }

                            // Handle ACK (established connections)
                            if ack == 1 {
                                log_new_connection(ctx, 0, dst_port, 6, 1);
                                return Ok(xdp_action::XDP_PASS);
                            }

                            // For other TCP packets, apply the rule's action
                            if rule.action == 1 {
                                log_new_connection(ctx, 0, dst_port, 6, 1);
                                return Ok(xdp_action::XDP_PASS);
                            }

                            log_new_connection(ctx, 0, dst_port, 6, 0);
                            return Ok(xdp_action::XDP_DROP);
                        }
                        IpProto::Udp => {
                            let udphdr: *const UdpHdr =
                                unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)? };
                            let dst_port = u16::from_be(unsafe { (*udphdr).dest });

                            if rule.ports.contains(&dst_port) && rule.action == 1 {
                                log_new_connection(ctx, 0, dst_port, 17, 1);
                                return Ok(xdp_action::XDP_PASS);
                            }
                            return Ok(xdp_action::XDP_DROP);
                        }
                        IpProto::Icmp => {
                            if let Some(&icmp_ping) = ICMP_RULE.get(0) {
                                if icmp_ping == 1 {
                                    log_new_connection(ctx, 0, 0, 1, 1);
                                    return Ok(xdp_action::XDP_PASS);
                                }
                            }
                            return Ok(xdp_action::XDP_DROP);
                        }
                        _ => return Ok(xdp_action::XDP_DROP),
                    }
                }
            }
            Ok(xdp_action::XDP_DROP)
        }
        _ => Ok(xdp_action::XDP_DROP),
    }
}
