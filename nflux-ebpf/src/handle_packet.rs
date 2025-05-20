use core::mem;

use aya_ebpf::programs::TcContext;
use network_types::{
    eth::EthHdr,
    ip::{IpProto, Ipv4Hdr, Ipv6Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::IpFamily;

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
fn handle_ports(ctx: &TcContext, proto: IpProto, is_ether: bool) -> Result<(u16, u16), ()> {
    match proto {
        IpProto::Tcp => {
            let (src_port, dst_port);
            if is_ether {
                let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                unsafe {
                    src_port = u16::from_be((*tcphdr).source);
                    dst_port = u16::from_be((*tcphdr).dest);
                }
            } else {
                let tcphdr: TcpHdr = ctx.load(20).map_err(|_| ())?;
                src_port = u16::from_be(tcphdr.source);
                dst_port = u16::from_be(tcphdr.dest);
            }

            Ok((src_port, dst_port))
        }
        IpProto::Udp => {
            let (src_port, dst_port);

            if is_ether {
                let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                src_port = u16::from_be_bytes((unsafe { *udphdr }).source);
                dst_port = u16::from_be_bytes((unsafe { *udphdr }).dest);
            } else {
                let udphdr: UdpHdr = ctx.load(20).map_err(|_| ())?;
                src_port = u16::from_be_bytes(udphdr.source);
                dst_port = u16::from_be_bytes(udphdr.dest);
            }
            Ok((src_port, dst_port))
        }
        IpProto::Icmp => Ok((0, 0)),
        _ => Ok((0, 0)),
    }
}

pub fn handle_packet(
    ctx: &TcContext,
    direction: u8,
    header: IpHeader,
    is_ether: bool,
) -> Result<PacketData, ()> {
    match header {
        IpHeader::V4(ipv4hdr) => {
            let source = u32::from_be_bytes(ipv4hdr.src_addr);
            let destination = u32::from_be_bytes(ipv4hdr.dst_addr);
            let total_len = u16::from_be_bytes(ipv4hdr.tot_len);
            let ttl = u8::from_be(ipv4hdr.ttl);
            let proto = ipv4hdr.proto;
            let (src_port, dst_port) = handle_ports(ctx, proto, is_ether).unwrap_or((0, 0));

            let packet_data = PacketData {
                src_ip: source,
                dst_ip: destination,
                total_len,
                ttl,
                proto: IpProto::Tcp as u8,
                src_port,
                dst_port,
                ip_family: IpFamily::Ipv4,
                direction,
            };

            Ok(packet_data)
        }
        IpHeader::V6(_ipv6hdr) => {
            // let proto = ipv6hdr.next_hdr;
            // match proto {
            //     IpProto::Tcp => {
            //         // let _tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
            //         return Ok(TC_ACT_PIPE);
            //     }
            //     IpProto::Udp => {
            //         // let _udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
            //         return Ok(TC_ACT_PIPE);
            //     }
            //     IpProto::Icmp => {
            //         // let _icmphdr: *const IcmpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)?;
            //         return Ok(TC_ACT_PIPE);
            //     }
            //     _ => {}
            // }
            Ok(PacketData {
                src_ip: 0,
                dst_ip: 0,
                total_len: 0,
                ttl: 0,
                proto: IpProto::Icmp as u8,
                src_port: 0,
                dst_port: 0,
                ip_family: IpFamily::Ipv6,
                direction,
            })
        }
    }
}
