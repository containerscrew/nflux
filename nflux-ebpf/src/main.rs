#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]
use core::mem;

use aya_ebpf::{
    bindings::xdp_action::{self, XDP_PASS},
    helpers::generated::bpf_ktime_get_ns,
    macros::xdp,
    programs::XdpContext,
};
use network_types::{
    arp::ArpHdr,
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr, Ipv6Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{
    dto::{ActiveConnectionKey, ArpEvent, IpFamily, NetworkEvent, TcpFlags},
    maps::{ACTIVE_CONNECTIONS, ARP_EVENTS, CONFIGMAP, NETWORK_EVENT},
};

#[xdp]
pub fn xdp_program(ctx: XdpContext) -> u32 {
    match unsafe { try_xdp_program(ctx) } {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(
    ctx: &XdpContext,
    offset: usize,
) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[inline(always)]
unsafe fn try_xdp_program(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    let config = CONFIGMAP.get(0).ok_or(())?;

    match (*ethhdr).ether_type() {
        Ok(EtherType::Ipv4) => {
            let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let mut src_ip = [0u8; 16];
            let mut dst_ip = [0u8; 16];

            let r_src_ip = unsafe { (*ipv4hdr).src_addr };
            let r_dst_ip = unsafe { (*ipv4hdr).dst_addr };

            src_ip[12..].copy_from_slice(&r_src_ip);
            dst_ip[12..].copy_from_slice(&r_dst_ip);

            let total_len = u16::from_be_bytes(unsafe { (*ipv4hdr).tot_len });
            let protocol = unsafe { (*ipv4hdr).proto };
            let ttl = unsafe { (*ipv4hdr).ttl };
            let mut src_port: u16 = 0;
            let mut dst_port: u16 = 0;
            let mut tcp_flags: Option<TcpFlags> = None;

            match protocol {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    src_port = u16::from_be_bytes(unsafe { (*tcphdr).source });
                    dst_port = u16::from_be_bytes(unsafe { (*tcphdr).dest });
                    tcp_flags = Some(TcpFlags {
                        syn: ((*tcphdr).syn() != 0) as u8,
                        ack: ((*tcphdr).ack() != 0) as u8,
                        fin: ((*tcphdr).fin() != 0) as u8,
                        rst: ((*tcphdr).rst() != 0) as u8,
                        psh: ((*tcphdr).psh() != 0) as u8,
                        urg: ((*tcphdr).urg() != 0) as u8,
                        ece: ((*tcphdr).ece() != 0) as u8,
                        cwr: ((*tcphdr).cwr() != 0) as u8,
                    });

                    if config.enable_tcp == 0 {
                        return Ok(XDP_PASS);
                    }
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    src_port = u16::from_be_bytes(unsafe { (*udphdr).src });
                    dst_port = u16::from_be_bytes(unsafe { (*udphdr).dst });
                    tcp_flags = None;

                    if config.enable_udp == 0 {
                        return Ok(XDP_PASS);
                    }
                }
                IpProto::Icmp => {}
                _ => return Ok(XDP_PASS),
            }

            if config.listen_port != 0
                && (src_port != config.listen_port && dst_port != config.listen_port)
            {
                return Ok(XDP_PASS);
            }

            // Check if the active connection is already tracked
            let key = ActiveConnectionKey {
                protocol: protocol as u8,
                src_port,
                dst_port,
                src_ip,
                dst_ip,
            };

            let current_time = bpf_ktime_get_ns();
            let log_interval = config.log_interval;

            if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
                if current_time - *last_log_time < log_interval {
                    return Ok(XDP_PASS);
                }
            }

            ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();

            if let Some(mut data) = NETWORK_EVENT.reserve::<NetworkEvent>(0) {
                let event = NetworkEvent {
                    src_ip,
                    dst_ip,
                    total_len,
                    ttl,
                    src_port,
                    dst_port,
                    protocol: protocol as u8,
                    direction: 0,
                    ip_family: nflux_common::dto::IpFamily::Ipv4,
                    tcp_flags,
                };
                data.write(event);
                data.submit(0);
            }
        }
        Ok(EtherType::Ipv6) => {
            let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let src_ip = unsafe { (*ipv6hdr).src_addr };
            let dst_ip = unsafe { (*ipv6hdr).dst_addr };
            let total_len = u16::from_be_bytes(unsafe { (*ipv6hdr).payload_len });
            let protocol = unsafe { (*ipv6hdr).next_hdr };
            let ttl = unsafe { (*ipv6hdr).hop_limit };
            let mut src_port: u16 = 0;
            let mut dst_port: u16 = 0;
            let mut tcp_flags: Option<TcpFlags> = None;

            match protocol {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    src_port = u16::from_be_bytes(unsafe { (*tcphdr).source });
                    dst_port = u16::from_be_bytes(unsafe { (*tcphdr).dest });

                    tcp_flags = Some(TcpFlags {
                        syn: ((*tcphdr).syn() != 0) as u8,
                        ack: ((*tcphdr).ack() != 0) as u8,
                        fin: ((*tcphdr).fin() != 0) as u8,
                        rst: ((*tcphdr).rst() != 0) as u8,
                        psh: ((*tcphdr).psh() != 0) as u8,
                        urg: ((*tcphdr).urg() != 0) as u8,
                        ece: ((*tcphdr).ece() != 0) as u8,
                        cwr: ((*tcphdr).cwr() != 0) as u8,
                    });

                    if config.enable_tcp == 0 {
                        return Ok(XDP_PASS);
                    }
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    src_port = u16::from_be_bytes(unsafe { (*udphdr).src });
                    dst_port = u16::from_be_bytes(unsafe { (*udphdr).dst });
                    tcp_flags = None;

                    if config.enable_udp == 0 {
                        return Ok(XDP_PASS);
                    }
                }
                IpProto::Icmp => {}
                _ => return Ok(XDP_PASS),
            }

            if config.listen_port != 0
                && (src_port != config.listen_port && dst_port != config.listen_port)
            {
                return Ok(XDP_PASS);
            }

            // Check if the active connection is already tracked
            let key = ActiveConnectionKey {
                protocol: protocol as u8,
                src_port,
                dst_port,
                src_ip,
                dst_ip,
            };

            let current_time = bpf_ktime_get_ns();
            let log_interval = config.log_interval;

            if let Some(last_log_time) = ACTIVE_CONNECTIONS.get(&key) {
                if current_time - *last_log_time < log_interval {
                    return Ok(XDP_PASS);
                }
            }

            ACTIVE_CONNECTIONS.insert(&key, &current_time, 0).ok();

            if let Some(mut data) = NETWORK_EVENT.reserve::<NetworkEvent>(0) {
                let event = NetworkEvent {
                    src_ip,
                    dst_ip,
                    total_len,
                    ttl,
                    src_port,
                    dst_port,
                    protocol: protocol as u8,
                    direction: 0,
                    ip_family: nflux_common::dto::IpFamily::Ipv4,
                    tcp_flags,
                };
                data.write(event);
                data.submit(0);
            }
        }
        Ok(EtherType::Arp) => {
            let arphdr: *const ArpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let op_code = u16::from_be_bytes(unsafe { (*arphdr).oper });

            let ip_family = match u16::from_be_bytes(unsafe { (*arphdr).ptype }) {
                0x0800 => IpFamily::Ipv4, // AF_INET
                0x86DD => IpFamily::Ipv6, // AF_INET6
                _ => IpFamily::Unknown,
            };

            if config.enable_arp == 0 {
                return Ok(XDP_PASS);
            }

            if let Some(mut slot) = ARP_EVENTS.reserve(0) {
                slot.write(ArpEvent { op_code, ip_family });
                slot.submit(0);
            }
        }
        // Err(_) => return Ok(XDP_PASS),
        _ => {
            // Other packet
            return Ok(XDP_PASS);
        }
    }

    Ok(xdp_action::XDP_PASS)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
