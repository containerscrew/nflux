use core::mem;

use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::{
    eth::EthHdr,
    ip::{IpProto, Ipv4Hdr, Ipv6Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{IpFamily, TcEvent};

use crate::{maps::TC_CONFIG, tc_event::log_connection};

pub enum IpHeader {
    V4(Ipv4Hdr),
    V6(Ipv6Hdr),
}

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

/// function to extract the source and destination ports from the TCP/UDP headers
/// So redundant code by the moment, but works
fn handle_ports(
    ctx: &TcContext,
    protocol: IpProto,
    is_ether: bool,
    ip_family: IpFamily,
) -> Result<(u16, u16), ()> {

    // Determine the offset based on whether it's an Ethernet frame and the IP family
    let offset = match (is_ether, ip_family) {
        (true, IpFamily::Ipv4) => EthHdr::LEN + Ipv4Hdr::LEN,
        (true, IpFamily::Ipv6) => EthHdr::LEN + Ipv6Hdr::LEN,
        (false, IpFamily::Ipv4) => Ipv4Hdr::LEN,
        (false, IpFamily::Ipv6) => Ipv6Hdr::LEN,
    };

    match protocol {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr = ptr_at(ctx, offset)?;
            unsafe {
                let src_port = u16::from_be((*tcphdr).source);
                let dst_port = u16::from_be((*tcphdr).dest);
                Ok((src_port, dst_port))
            }
        }
        IpProto::Udp => {
            let udphdr: *const UdpHdr = ptr_at(ctx, offset)?;
            unsafe {
                let src_port = u16::from_be_bytes((*udphdr).source);
                let dst_port = u16::from_be_bytes((*udphdr).dest);
                Ok((src_port, dst_port))
            }
        }
        _ => Ok((0, 0)), // ICMP
    }
}

pub fn handle_packet(
    ctx: &TcContext,
    direction: u8,
    header: IpHeader,
    l2: bool,
) -> Result<i32, ()> {
    // Load runtime config from eBPF map
    let tc_config = TC_CONFIG.get(0).ok_or(())?;

    match header {
        IpHeader::V4(ipv4hdr) => {
            let mut src_ip = [0u8; 16];
            src_ip[12..].copy_from_slice(&ipv4hdr.src_addr); // header only has 32 bits (4ytes)
            let mut dst_ip = [0u8; 16];
            dst_ip[12..].copy_from_slice(&ipv4hdr.dst_addr);
            let total_len = u16::from_be_bytes(ipv4hdr.tot_len);
            let ttl = ipv4hdr.ttl;
            let protocol = ipv4hdr.proto;

            let (src_port, dst_port) =
                handle_ports(ctx, protocol, l2, IpFamily::Ipv4).unwrap_or((0, 0));

            let event = TcEvent {
                src_ip,
                dst_ip,
                total_len,
                ttl,
                src_port,
                dst_port,
                protocol: protocol as u8,
                direction,
                ip_family: IpFamily::Ipv4,
            };

            unsafe {
                log_connection(&event, *tc_config);
            }

            Ok(TC_ACT_PIPE)
        }
        IpHeader::V6(ipv6hdr) => {
            let source = ipv6hdr.src_addr().octets(); // header already has 128 bits (16 bytes)
            let destination = ipv6hdr.dst_addr().octets();
            let proto = ipv6hdr.next_hdr;
            let (src_port, dst_port) =
                handle_ports(ctx, proto, l2, IpFamily::Ipv6).unwrap_or((0, 0));

            let event = TcEvent {
                src_ip: source,
                dst_ip: destination,
                total_len: u16::from_be_bytes(ipv6hdr.payload_len),
                ttl: ipv6hdr.hop_limit,
                src_port,
                dst_port,
                protocol: proto as u8,
                direction,
                ip_family: IpFamily::Ipv6,
            };

            unsafe {
                log_connection(&event, *tc_config);
            }

            Ok(TC_ACT_PIPE)
        }
    }
}
