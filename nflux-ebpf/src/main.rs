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
    dto::{ActiveConnectionKey, ArpEvent, FlowState, IpFamily, NetworkEvent, TcpFlags},
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

fn handle_ports(
    ctx: &XdpContext,
    protocol: IpProto,
    l4_offset: usize,
) -> Result<(u16, u16, Option<TcpFlags>), ()> {
    match protocol {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, l4_offset)? };
            unsafe {
                let src_port = u16::from_be_bytes((*tcphdr).source);
                let dst_port = u16::from_be_bytes((*tcphdr).dest);

                let tcp_flags = TcpFlags {
                    syn: ((*tcphdr).syn() != 0) as u8,
                    ack: ((*tcphdr).ack() != 0) as u8,
                    fin: ((*tcphdr).fin() != 0) as u8,
                    rst: ((*tcphdr).rst() != 0) as u8,
                    psh: ((*tcphdr).psh() != 0) as u8,
                    urg: ((*tcphdr).urg() != 0) as u8,
                    ece: ((*tcphdr).ece() != 0) as u8,
                    cwr: ((*tcphdr).cwr() != 0) as u8,
                };

                Ok((src_port, dst_port, Some(tcp_flags)))
            }
        }
        IpProto::Udp => {
            let udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, l4_offset)? };
            unsafe {
                let src_port = u16::from_be_bytes((*udphdr).src);
                let dst_port = u16::from_be_bytes((*udphdr).dst);
                Ok((src_port, dst_port, None))
            }
        }
        _ => Ok((0, 0, None)),
    }
}

#[inline(always)]
fn emit_event(
    src_ip: [u8; 16],
    dst_ip: [u8; 16],
    total_len: u16,
    ttl: u8,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    ip_family: IpFamily,
    tcp_flags: Option<TcpFlags>,
    direction: u8,
) {
    if let Some(mut data) = NETWORK_EVENT.reserve::<NetworkEvent>(0) {
        let event = NetworkEvent {
            src_ip,
            dst_ip,
            total_len,
            ttl,
            src_port,
            dst_port,
            protocol,
            direction,
            ip_family,
            tcp_flags,
        };
        data.write(event);
        data.submit(0);
    }
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
            src_ip[12..].copy_from_slice(&(*ipv4hdr).src_addr);
            dst_ip[12..].copy_from_slice(&(*ipv4hdr).dst_addr);

            let total_len = u16::from_be_bytes((*ipv4hdr).tot_len);
            let protocol = (*ipv4hdr).proto;
            let ttl = (*ipv4hdr).ttl;

            if (protocol == IpProto::Tcp && config.enable_tcp == 0)
                || (protocol == IpProto::Udp && config.enable_udp == 0)
            {
                return Ok(XDP_PASS);
            }

            let ihl = ((*ipv4hdr).vihl & 0x0f) as usize;
            let l4_offset = EthHdr::LEN + ihl * 4;

            let (src_port, dst_port, tcp_flags) = handle_ports(&ctx, protocol, l4_offset)?;

            if config.listen_port != 0
                && src_port != config.listen_port
                && dst_port != config.listen_port
            {
                return Ok(XDP_PASS);
            }

            let key = ActiveConnectionKey {
                protocol: protocol as u8,
                src_port,
                dst_port,
                src_ip,
                dst_ip,
            };

            let had_entry = ACTIVE_CONNECTIONS.get(&key).is_some();

            let now = bpf_ktime_get_ns();
            let bytes = total_len as u64;

            match ACTIVE_CONNECTIONS.get(&key) {
                Some(st) => {
                    let mut new_st = *st;
                    new_st.last_seen_ns = now;
                    new_st.packets += 1;
                    new_st.bytes += bytes;
                    ACTIVE_CONNECTIONS.insert(&key, &new_st, 0).ok();
                }
                None => {
                    let st = FlowState {
                        first_seen_ns: now,
                        last_seen_ns: now,
                        packets: 1,
                        bytes,
                    };
                    ACTIVE_CONNECTIONS.insert(&key, &st, 0).ok();
                }
            }

            let direction = if config.listen_port != 0 && dst_port == config.listen_port {
                0
            } else if config.listen_port != 0 && src_port == config.listen_port {
                1
            } else {
                0
            };

            let mut should_emit = false;

            if protocol == IpProto::Tcp {
                if let Some(f) = tcp_flags {
                    if !had_entry && f.syn != 0 && f.ack == 0 {
                        should_emit = true;
                    } else if had_entry && (f.fin != 0 || f.rst != 0) {
                        should_emit = true;
                    }
                }
            } else if protocol == IpProto::Udp {
                if !had_entry {
                    should_emit = true;
                }
            }

            if should_emit {
                emit_event(
                    src_ip,
                    dst_ip,
                    total_len,
                    ttl,
                    src_port,
                    dst_port,
                    protocol as u8,
                    IpFamily::Ipv4,
                    tcp_flags,
                    direction,
                );

                if protocol == IpProto::Tcp {
                    if let Some(f) = tcp_flags {
                        if f.fin != 0 || f.rst != 0 {
                            ACTIVE_CONNECTIONS.remove(&key).ok();
                        }
                    }
                }
            }
        }

        Ok(EtherType::Ipv6) => {
            let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };

            let src_ip = (*ipv6hdr).src_addr;
            let dst_ip = (*ipv6hdr).dst_addr;
            let total_len = u16::from_be_bytes((*ipv6hdr).payload_len);
            let protocol = (*ipv6hdr).next_hdr;
            let ttl = (*ipv6hdr).hop_limit;

            if protocol != IpProto::Tcp && protocol != IpProto::Udp {
                return Ok(XDP_PASS);
            }

            if (protocol == IpProto::Tcp && config.enable_tcp == 0)
                || (protocol == IpProto::Udp && config.enable_udp == 0)
            {
                return Ok(XDP_PASS);
            }

            let l4_offset = EthHdr::LEN + Ipv6Hdr::LEN;
            let (src_port, dst_port, tcp_flags) = handle_ports(&ctx, protocol, l4_offset)?;

            if config.listen_port != 0
                && src_port != config.listen_port
                && dst_port != config.listen_port
            {
                return Ok(XDP_PASS);
            }

            let key = ActiveConnectionKey {
                protocol: protocol as u8,
                src_port,
                dst_port,
                src_ip,
                dst_ip,
            };

            let had_entry = ACTIVE_CONNECTIONS.get(&key).is_some();

            let now = bpf_ktime_get_ns();
            let bytes = total_len as u64;

            match ACTIVE_CONNECTIONS.get(&key) {
                Some(st) => {
                    let mut new_st = *st;
                    new_st.last_seen_ns = now;
                    new_st.packets += 1;
                    new_st.bytes += bytes;
                    ACTIVE_CONNECTIONS.insert(&key, &new_st, 0).ok();
                }
                None => {
                    let st = FlowState {
                        first_seen_ns: now,
                        last_seen_ns: now,
                        packets: 1,
                        bytes,
                    };
                    ACTIVE_CONNECTIONS.insert(&key, &st, 0).ok();
                }
            }

            let direction = if config.listen_port != 0 && dst_port == config.listen_port {
                0
            } else if config.listen_port != 0 && src_port == config.listen_port {
                1
            } else {
                0
            };

            let mut should_emit = false;

            if protocol == IpProto::Tcp {
                if let Some(f) = tcp_flags {
                    if !had_entry && f.syn != 0 && f.ack == 0 {
                        should_emit = true;
                    } else if had_entry && (f.fin != 0 || f.rst != 0) {
                        should_emit = true;
                    }
                }
            } else if protocol == IpProto::Udp {
                if !had_entry {
                    should_emit = true;
                }
            }

            if should_emit {
                emit_event(
                    src_ip,
                    dst_ip,
                    total_len,
                    ttl,
                    src_port,
                    dst_port,
                    protocol as u8,
                    IpFamily::Ipv6,
                    tcp_flags,
                    direction,
                );

                if protocol == IpProto::Tcp {
                    if let Some(f) = tcp_flags {
                        if f.fin != 0 || f.rst != 0 {
                            ACTIVE_CONNECTIONS.remove(&key);
                        }
                    }
                }
            }
        }

        Ok(EtherType::Arp) => {
            let arphdr: *const ArpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let op_code = u16::from_be_bytes((*arphdr).oper);

            let ip_family = match u16::from_be_bytes((*arphdr).ptype) {
                0x0800 => IpFamily::Ipv4,
                0x86DD => IpFamily::Ipv6,
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

        _ => {}
    }

    Ok(XDP_PASS)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
