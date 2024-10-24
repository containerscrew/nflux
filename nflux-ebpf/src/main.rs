#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{Array, LruHashMap, HashMap},
    programs::XdpContext,
};
use aya_log_ebpf::{debug, info};
use nflux_common::{MAX_ALLOWED_IPV4, MAX_ALLOWED_PORTS, MAX_FIREWALL_RULES, MAX_RULES_PORT};

use core::mem;
use aya_ebpf::helpers::bpf_ktime_get_ns;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{Ipv4Hdr, IpProto},
    tcp::TcpHdr,
    udp::UdpHdr,
    icmp::IcmpHdr,
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
#[map]
static ALLOWED_PORTS: Array<u32> = Array::with_max_entries(MAX_ALLOWED_PORTS as u32, 0);
#[map]
static ALLOWED_IPV4: Array<u32> = Array::with_max_entries(MAX_ALLOWED_IPV4 as u32, 0);

#[repr(C)]
struct IpPort {
    ip: u32,
    port: u16,
}

#[map]
static RECENT_LOGS: LruHashMap<IpPort, u64> = LruHashMap::with_max_entries(1024, 0);

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

// Check if a port is allowed
fn is_port_allowed(port: u16) -> bool {
    for i in 0..MAX_ALLOWED_PORTS as u32 {
        if let Some(allowed_port) = ALLOWED_PORTS.get(i) {
            if port as u32 == *allowed_port {
                return true;
            }
        }
    }
    false
}

// Check if an IP address is allowed
fn is_ipv4_allowed(ip: u32) -> bool {
    for i in 0..MAX_ALLOWED_IPV4 as u32 {
        if let Some(&allowed_ip) = ALLOWED_IPV4.get(i) {
            if ip == allowed_ip {
                return true;
            }
        }
    }
    false
}

// Helper function to get the current time in nanoseconds
fn current_time_ns() -> u64 {
    unsafe { bpf_ktime_get_ns() }
}

// Function to check if we should log a dropped SYN packet to avoid excessive logging
unsafe fn should_log(ip: u32, port: u16) -> bool {
    let key = IpPort { ip, port };
    let now = current_time_ns();

    if let Some(&last_logged) = RECENT_LOGS.get(&key) {
        // Only log if more than 5 second has passed
        if now - last_logged < 5_000_000_000 {
            return false;
        }
    }

    // Update the map with the new timestamp
    RECENT_LOGS.insert(&key, &now, 0).ok();
    true
}

fn start_nflux(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    return match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {
            let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let source = u32::from_be(unsafe { (*ipv4hdr).src_addr });
            let proto = unsafe { (*ipv4hdr).proto };

            match proto {
                IpProto::Tcp => unsafe {
                    // Parse the TCP header
                    let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*tcphdr).dest });

                    // Check allowed ports
                    if is_port_allowed(dst_port) {
                        info!(&ctx, "Allowed incoming connection to port: {} from: {:i}", dst_port, source);
                        return Ok(xdp_action::XDP_PASS);
                    }

                    // Check if the IP address is blocked
                    if is_ipv4_allowed(source) {
                        info!(&ctx, "Allowed incoming connection from IP: {:i}", source);
                        return Ok(xdp_action::XDP_PASS);
                    }

                    // Deny incoming connections instead syn-ack packets to allow using browsing or other outgoing TCP connections the user did
                    if unsafe { (*tcphdr).syn() == 1 && (*tcphdr).ack() == 0 } {
                        if should_log(source, dst_port) {
                            info!(
                                &ctx,
                                "TCP SYN packet dropped (new incoming connection): from IP: {:i}, to local port: {}",
                                source,
                                dst_port
                            );
                        }
                        return Ok(xdp_action::XDP_DROP);
                    }
                    debug!(&ctx, "TCP SYN-ACK packet accepted: from IP: {:i}, to local port: {}", source, dst_port);
                    Ok(xdp_action::XDP_DROP) // Adjust as needed
                }
                IpProto::Udp => {
                    // Parse UDP header
                    let udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*udphdr).dest });
                    debug!(&ctx, "UDP Packet: SRC IP {:i}, DST PORT {}", source, dst_port);

                    // Add logic to allow/deny UDP based on firewall rules

                    Ok(xdp_action::XDP_PASS) // Adjust as needed
                }
                IpProto::Icmp => {
                    // Parse ICMP header
                    let icmphdr: *const IcmpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let icmp_type = unsafe { (*icmphdr).type_ };
                    debug!(&ctx, "ICMP Packet: SRC IP {:i}, TYPE {}", source, icmp_type);

                    // Add logic to allow/deny ICMP based on rules

                    Ok(xdp_action::XDP_DROP) // Adjust as needed
                }
                _ => {
                    // For other protocols, drop by default
                    Ok(xdp_action::XDP_PASS)
                }
            }
        }
        _ => Ok(xdp_action::XDP_PASS), // Drop non-IPv4 packets
    }
}
