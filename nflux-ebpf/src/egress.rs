use core::mem;

use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::helpers::gen::bpf_get_current_pid_tgid;
use aya_ebpf::programs::TcContext;
use aya_log_ebpf::debug;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use network_types::tcp::TcpHdr;
use network_types::udp::UdpHdr;
use nflux_common::EgressEvent;

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
unsafe fn log_connection(ctx: &TcContext, log_new_connection: u8, destination: u32, src_port: u16, dst_port: u16, protocol: u8, pid: u64) {
    // If log_only_new_connections is enabled
    // Only log connections to different endpoints (ips)
    match log_new_connection{
        0 => {
            // Log all connections
            let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
            EGRESS_EVENT.output(ctx, &event, 0);
        }
        1 => {
            // Check if this destination is already active
            if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
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

pub fn try_tc_egress(ctx: TcContext) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;

    let egress_config = EGRESS_CONFIG.get(0).ok_or(())?;

    match ethhdr.ether_type {
        EtherType::Ipv4 => unsafe {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            let destination = u32::from_be(ipv4hdr.dst_addr);

            match ipv4hdr.proto {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                    let src_port = u16::from_be((*tcphdr).source);
                    let dst_port = u16::from_be((*tcphdr).dest);
                    let protocol = IpProto::Tcp as u8;
                    let pid_tgid = bpf_get_current_pid_tgid();
                    let pid = pid_tgid >> 32;

                    // If log_tcp_connections is enabled, log the connection
                    if egress_config.log_tcp_connections == 1 {
                        log_connection(&ctx, egress_config.log_only_new_connections, destination, src_port, dst_port, protocol, pid);
                    }
                    return Ok(TC_ACT_PIPE)
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                    let src_port = u16::from_be((*udphdr).source);
                    let dst_port = u16::from_be((*udphdr).dest);
                    let protocol = IpProto::Udp as u8;
                    let pid_tgid = bpf_get_current_pid_tgid();
                    let pid = pid_tgid >> 32;

                    // If log_udp_connections is enabled, log the connection
                    if egress_config.log_udp_connections == 1 {
                        log_connection(&ctx, egress_config.log_only_new_connections, destination, src_port, dst_port, protocol, pid);
                    }

                    return Ok(TC_ACT_PIPE)
                }
                _ => {}
            }
        }
        EtherType::Ipv6 => {
            // Ipv6 not implemented yet
            debug!(&ctx, "is an ipv6 packet!");
            return Ok(TC_ACT_PIPE)
        }
        EtherType::FibreChannel => {
            debug!(&ctx, "ether type fibrechannel!!");
            return Ok(TC_ACT_PIPE)
        }
        EtherType::Arp => {
            debug!(&ctx, "ARP packet!!");
            return Ok(TC_ACT_PIPE)
        }
        _ => {
            debug!(&ctx, "Unknown ether type.");
            return Ok(TC_ACT_PIPE)
        },
    }

    Ok(TC_ACT_PIPE)
}
