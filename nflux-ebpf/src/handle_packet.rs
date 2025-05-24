use core::mem;

use aya_ebpf::programs::TcContext;
use network_types::{
    eth::EthHdr,
    ip::{IpProto, Ipv4Hdr, Ipv6Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{IpFamily, TcEvent};

pub enum IpHeader {
    V4(Ipv4Hdr),
    V6(Ipv6Hdr),
}

pub struct PacketData {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub total_len: u16,
    pub ttl: u8,
    pub proto: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub ip_family: IpFamily,
    pub direction: u8,
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
    is_ether: bool,
    src_mac: [u8; 6],
    dst_mac: [u8; 6],
) -> Result<TcEvent, ()> {
    match header {
        IpHeader::V4(ipv4hdr) => {
            let src_ip = u32::from_be_bytes(ipv4hdr.src_addr);
            let dst_ip = u32::from_be_bytes(ipv4hdr.dst_addr);
            let total_len = u16::from_be_bytes(ipv4hdr.tot_len);
            let ttl = u8::from_be(ipv4hdr.ttl);
            let protocol = ipv4hdr.proto;
            let (src_port, dst_port) =
                handle_ports(ctx, protocol, is_ether, IpFamily::Ipv4).unwrap_or((0, 0));

            let event = TcEvent {
                src_mac,
                dst_mac,
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

            Ok(event)
        }
        IpHeader::V6(ipv6hdr) => {
            let source = ipv6hdr.src_addr().to_bits();
            let destionation = ipv6hdr.dst_addr().to_bits();
            let proto = ipv6hdr.next_hdr;
            let (src_port, dst_port) =
                handle_ports(ctx, proto, is_ether, IpFamily::Ipv6).unwrap_or((0, 0));

            let event = TcEvent {
                src_mac,
                dst_mac,
                src_ip: source as u32,
                dst_ip: destionation as u32,
                total_len: u16::from_be_bytes(ipv6hdr.payload_len),
                ttl: ipv6hdr.hop_limit,
                src_port,
                dst_port,
                protocol: proto as u8,
                direction,
                ip_family: IpFamily::Ipv6,
            };

            Ok(event)
        }
    }
}
