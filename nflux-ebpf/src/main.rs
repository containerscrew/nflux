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
use maps::{CONNECTION_EVENTS, CONNECTION_TRACKER, ICMP_RULE, IPV4_RULES};
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
        _ => Ok(XDP_DROP),
    }
}

fn process_ipv4(ctx: &XdpContext) -> Result<u32, ()> {
    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(ctx, EthHdr::LEN)? };
    let source_ip = u32::from_be(unsafe { (*ipv4hdr).src_addr });
    let dest_ip = u32::from_be(unsafe { (*ipv4hdr).dst_addr });
    let proto = unsafe { (*ipv4hdr).proto };

    // Use LPM map to find the longest prefix match for the source IP
    if let Some(rule) = IPV4_RULES.get(&Key::new(32, LpmKeyIpv4 { prefix_len: 32, ip: source_ip })) {
        match proto {
            IpProto::Tcp => {
                let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
                let syn = unsafe { (*tcphdr).syn() };
                let ack = unsafe { (*tcphdr).ack() };
                let fin = unsafe { (*tcphdr).fin() };
                let rst = unsafe { (*tcphdr).rst() };

                let connection_key = ((source_ip as u64) << 32) | (dst_port as u64);

                if rule.ports.contains(&dst_port) && rule.action == 1 {
                    if ack == 1 {
                        // Check for active connections
                        if let Some(&last_seen) = unsafe { CONNECTION_TRACKER.get(&connection_key) } {
                            let current_time = unsafe { bpf_ktime_get_ns() };
                            let timeout_ns = 5 * 60 * 1_000_000_000; // 5 minutes

                            if current_time - last_seen > timeout_ns {
                                // Remove stale connection
                                let _ = CONNECTION_TRACKER.remove(&connection_key);
                                log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                                return Ok(XDP_DROP);
                            }

                            // Refresh timestamp for active connection
                            let _ = CONNECTION_TRACKER.insert(&connection_key, &current_time, 0);
                            return Ok(XDP_PASS);
                        }

                        // Unknown connection, drop it
                        log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
                        return Ok(XDP_DROP);
                    }

                    if syn == 1 && ack == 0 {
                        // New connection attempt (SYN)
                        let timestamp = unsafe { bpf_ktime_get_ns() };
                        let _ = CONNECTION_TRACKER.insert(&connection_key, &timestamp, 0);
                        log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 1);
                        return Ok(XDP_PASS);
                    }

                    if fin == 1 || rst == 1 {
                        // Connection close (FIN/RST)
                        let _ = CONNECTION_TRACKER.remove(&connection_key);
                        return Ok(XDP_PASS);
                    }
                }

                // Drop packets that are not part of any known connection
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
            _ => Ok(XDP_DROP),
        }
    } else {
        Ok(XDP_DROP)
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
