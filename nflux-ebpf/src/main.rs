#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

use aya_ebpf::helpers::bpf_ktime_get_ns;
use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{LruHashMap, PerfEventArray},
    programs::XdpContext,
};

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};

use nflux_common::{ConnectionEvent, Ipv4Rule};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// #[map]
// static GLOBAL_FIREWALL_RULES: Array<GlobalFirewallRules> = Array::with_max_entries(1, 0);

#[map]
static IPV4_RULES: LruHashMap<u32, Ipv4Rule> = LruHashMap::with_max_entries(1024, 0);

#[map]
static CONNECTION_EVENTS: PerfEventArray<ConnectionEvent> = PerfEventArray::new(0);

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
fn is_port_allowed(global_firewall_rules: &GlobalFirewallRules, port: u16) -> bool {
    for &allowed_port in &global_firewall_rules.allowed_ports {
        if allowed_port == 0 {
            // Stop if we hit an uninitialized entry (assuming 0 indicates unused entries)
            break;
        }
        if port as u32 == allowed_port {
            return true;
        }
    }
    false
}

// Check if an IP address is allowed
fn is_ipv4_allowed(app_config: &GlobalFirewallRules, ip: u32) -> bool {
    for &allowed_ip in &app_config.allowed_ipv4 {
        if allowed_ip == 0 {
            // Stop if we hit an uninitialized entry (assuming 0 indicates unused entries)
            break;
        }
        if ip == allowed_ip {
            return true;
        }
    }
    false
}

// Helper function to get the current time in nanoseconds
fn current_time_ns() -> u64 {
    unsafe { bpf_ktime_get_ns() }
}

#[repr(C)]
struct IpPort {
    ip: u32,
    port: u16,
}

#[map]
static RECENT_LOGS: LruHashMap<IpPort, u64> = LruHashMap::with_max_entries(1024, 0);

// Function to check if we should log a dropped SYN packet to avoid excessive logging
fn should_log(ip: u32, port: u16, log_interval_secs: u64) -> bool {
    let key = IpPort { ip, port };
    let now = current_time_ns();

    unsafe {
        if let Some(&last_logged) = RECENT_LOGS.get(&key) {
            // Only log if more than 5 seconds have passed
            if now - last_logged < log_interval_secs * 1_000_000_000 {
                return false;
            }
        }
    }

    // Update the map with the new timestamp
    RECENT_LOGS.insert(&key, &now, 0).ok();
    true
}

fn get_global_firewall_rules() -> &'static GlobalFirewallRules {
    GLOBAL_FIREWALL_RULES.get(0).unwrap()
}

fn log_new_connection(
    ctx: XdpContext,
    src_addr: u32,
    dst_port: u16,
    protocol: u8,
    log_interval_secs: u64,
) {
    let event = ConnectionEvent {
        src_addr,
        dst_port,
        protocol,
    };

    if should_log(src_addr, dst_port, log_interval_secs) {
        CONNECTION_EVENTS.output(&ctx, &event, 0);
    }
}

fn start_nflux(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    // Get global firewall rules
    let global_firewall_rules = get_global_firewall_rules();

    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => {
            let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let source = u32::from_be(unsafe { (*ipv4hdr).src_addr });
            let proto = unsafe { (*ipv4hdr).proto };

            match proto {
                IpProto::Tcp => {
                    // Parse the TCP header
                    let tcphdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*tcphdr).dest });

                    // if is_port_allowed(&global_firewall_rules, dst_port) {
                    //     log_new_connection(ctx, source, dst_port, 6, 5);
                    //     return Ok(xdp_action::XDP_PASS);
                    // }

                    // // Check if the IP address is allowed
                    // if is_ipv4_allowed(&global_firewall_rules, source) {
                    //     log_new_connection(ctx, source, dst_port, 6, 5);
                    //     return Ok(xdp_action::XDP_PASS);
                    // }

                    // // Deny incoming connections, except SYN-ACK packets
                    // if unsafe { (*tcphdr).syn() == 1 && (*tcphdr).ack() == 0 } {
                    //     // Block unsolicited incoming SYN packets (deny incoming connections)
                    //     return Ok(xdp_action::XDP_DROP);
                    // } else if unsafe { (*tcphdr).ack() == 1 } {
                    //     // Permit ACK packets (responses to outgoing connections)
                    //     log_new_connection(ctx, source, dst_port, 6, 5);
                    //     return Ok(xdp_action::XDP_PASS);
                    // }

                    Ok(xdp_action::XDP_DROP)
                }
                IpProto::Udp => {
                    // Parse UDP header
                    let udphdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    let dst_port = u16::from_be(unsafe { (*udphdr).dest });
                    let src_port = u16::from_be(unsafe { (*udphdr).source });

                    // If the source port (DNS) is 53, allow the packet. Internet connection
                    // if src_port == 53 {
                    //     return Ok(xdp_action::XDP_PASS);
                    // }

                    // // Check if the IP address is blocked
                    // if is_ipv4_allowed(&global_firewall_rules, source) {
                    //     log_new_connection(ctx, source, dst_port, 6, 5);
                    //     return Ok(xdp_action::XDP_PASS);
                    // }

                    // // Check allowed ports
                    // if is_port_allowed(&global_firewall_rules, dst_port) {
                    //     log_new_connection(ctx, source, dst_port, 6, 5);
                    //     return Ok(xdp_action::XDP_PASS);
                    // }

                    Ok(xdp_action::XDP_DROP)
                }
                // IpProto::Icmp => {
                //     if global_firewall_rules.allow_icmp == 1 {
                //         log_new_connection(ctx, source, 0, 1, 5);
                //         Ok(xdp_action::XDP_PASS)
                //     } else {
                //         Ok(xdp_action::XDP_DROP)
                //     }
                // }
                _ => Ok(xdp_action::XDP_DROP),
            }
        }
        _ => Ok(xdp_action::XDP_DROP),
    }
}
