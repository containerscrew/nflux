#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

use aya_ebpf::helpers::bpf_ktime_get_ns;
use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{Array, LruHashMap, PerfEventArray},
    programs::XdpContext,
};
use aya_log_ebpf::{debug, info};
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    icmp::IcmpHdr,
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{ConnectionEvent, MAX_ALLOWED_IPV4, MAX_ALLOWED_PORTS};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[map]
static ALLOWED_PORTS: Array<u32> = Array::with_max_entries(MAX_ALLOWED_PORTS as u32, 0);
#[map]
static ALLOWED_IPV4: Array<u32> = Array::with_max_entries(MAX_ALLOWED_IPV4 as u32, 0);

#[map]
static CONNECTION_EVENTS: PerfEventArray<ConnectionEvent> = PerfEventArray::new(0);

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
        // Only log if more than 5 seconds have passed
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
                    let tcphdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*tcphdr).dest });

                    // Check allowed ports
                    if is_port_allowed(dst_port) {
                        info!(
                            &ctx,
                            "Allowed incoming connection to port: {} from: {:i}", dst_port, source
                        );
                        return Ok(xdp_action::XDP_PASS);
                    }

                    // Check if the IP address is blocked
                    if is_ipv4_allowed(source) {
                        info!(&ctx, "Allowed incoming connection from IP: {:i}", source);
                        return Ok(xdp_action::XDP_PASS);
                    }

                    // Deny incoming connections, except SYN-ACK packets
                    if unsafe { (*tcphdr).syn() == 1 && (*tcphdr).ack() == 0 } {
                        if should_log(source, dst_port) {
                            info!(
                                &ctx,
                                "TCP SYN packet dropped (new incoming connection): from IP: {:i}, to local port: {}",
                                source,
                                dst_port
                            );
                        }
                        log_connection_event(&ctx, source, 0, 0, dst_port, 6); // Protocol 6 for TCP
                        return Ok(xdp_action::XDP_DROP);
                    }
                    debug!(
                        &ctx,
                        "TCP SYN-ACK packet accepted: from IP: {:i}, to local port: {}",
                        source,
                        dst_port
                    );
                    Ok(xdp_action::XDP_DROP)
                },
                IpProto::Udp => {
                    // Parse UDP header
                    let udphdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*udphdr).dest });
                    debug!(
                        &ctx,
                        "UDP Packet: SRC IP {:i}, DST PORT {}", source, dst_port
                    );

                    log_connection_event(&ctx, source, 0, 0, dst_port, 17); // Protocol 17 for UDP
                    Ok(xdp_action::XDP_PASS)
                }
                IpProto::Icmp => {
                    // Parse ICMP header
                    let icmphdr: *const IcmpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    debug!(&ctx, "ICMP Packet: SRC IP {:i}", source);
                    Ok(xdp_action::XDP_PASS)
                }
                _ => Ok(xdp_action::XDP_PASS),
            }
        }
        _ => Ok(xdp_action::XDP_PASS),
    };
}

// Log connection events for TCP and UDP
fn log_connection_event(
    ctx: &XdpContext,
    src_addr: u32,
    dst_addr: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
) {
    let event = ConnectionEvent {
        src_addr,
        dst_addr,
        src_port,
        dst_port,
        protocol,
    };
    let _ = CONNECTION_EVENTS.output(ctx, &event, 0);
}
