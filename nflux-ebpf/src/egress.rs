use core::mem;

use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::helpers::gen::bpf_get_current_pid_tgid;
use aya_ebpf::programs::TcContext;
use aya_log_ebpf::{debug, warn};
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use network_types::tcp::TcpHdr;
use network_types::udp::UdpHdr;
use nflux_common::{EgressConfig, EgressEvent};

use crate::maps::{ACTIVE_CONNECTIONS, EGRESS_CONFIG, EGRESS_EVENT};

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

#[inline]
unsafe fn log_connection(
    ctx: &TcContext,
    log_new_connection: u8,
    destination: u32,
    src_port: u16,
    dst_port: u16,
    protocol: u8,
    pid: u64,
) {
    // If log_only_new_connections is enabled
    // Only log connections to different endpoints (ips)
    match log_new_connection {
        0 => {
            // Log all connections
            let event = EgressEvent {
                dst_ip: destination,
                src_port,
                dst_port,
                protocol,
                pid,
            };
            EGRESS_EVENT.output(ctx, &event, 0);
        }
        1 => {
            // Check if this destination is already active
            if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                let event = EgressEvent {
                    dst_ip: destination,
                    src_port,
                    dst_port,
                    protocol,
                    pid,
                };
                EGRESS_EVENT.output(ctx, &event, 0);

                // Mark connection as active
                if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
                    return;
                }
            }
        }
        _ => {}
    }
}

fn handle_ipv4_packet(ctx: &TcContext, egress_config: &EgressConfig) -> Result<i32, ()> {
    let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
    let destination = u32::from_be(ipv4hdr.dst_addr);

    match ipv4hdr.proto {
        IpProto::Tcp => handle_tcp_packet(ctx, egress_config, destination),
        IpProto::Udp => handle_udp_packet(ctx, egress_config, destination),
        IpProto::Icmp => handle_icmp_packet(ctx, egress_config, destination),
        _ => Ok(TC_ACT_PIPE),
    }
}

fn handle_icmp_packet(
    ctx: &TcContext,
    egress_config: &EgressConfig,
    destination: u32,
) -> Result<i32, ()> {
    let pid_tgid = unsafe { bpf_get_current_pid_tgid() };
    let pid = pid_tgid >> 32;

    if egress_config.log_icmp_connections == 1 {
        unsafe {
            log_connection(
                ctx,
                egress_config.log_only_new_connections,
                destination,
                0,
                0,
                IpProto::Icmp as u8,
                pid,
            )
        };
    }

    Ok(TC_ACT_PIPE)
}


fn handle_tcp_packet(
    ctx: &TcContext,
    egress_config: &EgressConfig,
    destination: u32,
) -> Result<i32, ()> {
    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
    let src_port = u16::from_be((unsafe { *tcphdr }).source);
    let dst_port = u16::from_be((unsafe { *tcphdr }).dest);
    let protocol = IpProto::Tcp as u8;
    let pid_tgid = unsafe { bpf_get_current_pid_tgid() };
    let pid = pid_tgid >> 32;

    if egress_config.log_tcp_connections == 1 {
        unsafe {
            log_connection(
                ctx,
                egress_config.log_only_new_connections,
                destination,
                src_port,
                dst_port,
                protocol,
                pid,
            )
        };
    }

    Ok(TC_ACT_PIPE)
}

fn handle_udp_packet(
    ctx: &TcContext,
    egress_config: &EgressConfig,
    destination: u32,
) -> Result<i32, ()> {
    let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
    let src_port = u16::from_be((unsafe { *udphdr }).source);
    let dst_port = u16::from_be((unsafe { *udphdr }).dest);
    let protocol = IpProto::Udp as u8;
    let pid_tgid = unsafe { bpf_get_current_pid_tgid() };
    let pid = pid_tgid >> 32;

    if egress_config.log_udp_connections == 1 {
        unsafe {
            log_connection(
                ctx,
                egress_config.log_only_new_connections,
                destination,
                src_port,
                dst_port,
                protocol,
                pid,
            )
        };
    }

    Ok(TC_ACT_PIPE)
}

fn handle_non_ipv4_packet(ctx: &TcContext, ethhdr: &EthHdr) -> Result<i32, ()> {
    match ethhdr.ether_type {
        EtherType::Ipv6 => {
            debug!(ctx, "is an ipv6 packet!");
            Ok(TC_ACT_PIPE)
        }
        EtherType::FibreChannel => {
            debug!(ctx, "ether type fibrechannel!!");
            Ok(TC_ACT_PIPE)
        }
        EtherType::Arp => {
            debug!(ctx, "ARP packet!!");
            Ok(TC_ACT_PIPE)
        }
        _ => {
            debug!(ctx, "Unknown ether type.");
            Ok(TC_ACT_PIPE)
        }
    }
}

pub fn try_tc_egress_physical(ctx: TcContext) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    let egress_config = EGRESS_CONFIG.get(0).ok_or(())?;

    match ethhdr.ether_type {
        EtherType::Ipv4 => handle_ipv4_packet(&ctx, &egress_config),
        _ => handle_non_ipv4_packet(&ctx, &ethhdr),
    }
}

pub fn try_tc_egress_virtual(ctx: TcContext) -> Result<i32, ()> {
    // Parse IPv4 header
    let ipv4hdr: Ipv4Hdr = match ctx.load(0) {
        Ok(hdr) => hdr,
        Err(_) => {
            warn!(&ctx, "Failed to load IPv4 header");
            return Ok(TC_ACT_PIPE);
        }
    };

    let egress_config = EGRESS_CONFIG.get(0).ok_or(())?;
    let destination = u32::from_be(ipv4hdr.dst_addr);

    match ipv4hdr.proto {
        IpProto::Tcp => handle_tcp_packet(&ctx, egress_config, destination),
        IpProto::Udp => handle_udp_packet(&ctx, egress_config, destination),
        IpProto::Icmp => handle_icmp_packet(&ctx, egress_config, destination),
        _ => Ok(TC_ACT_PIPE),
    }
}
