use aya_ebpf::programs::XdpContext;
use network_types::{
    eth::EthHdr,
    ip::{IpProto, Ipv4Hdr, Ipv6Hdr}, tcp::TcpHdr, udp::UdpHdr,
};
use nflux_common::dto::{IpFamily, TcpFlags};

use crate::ptr_at;

pub fn handle_ports(
    ctx: &XdpContext,
    protocol: IpProto,
    ip_family: IpFamily,
) -> Result<(u16, u16, Option<TcpFlags>), ()> {
    // Calculate offset assuming Ethernet frame
    let offset = match ip_family {
        IpFamily::Ipv4 => EthHdr::LEN + Ipv4Hdr::LEN,
        IpFamily::Ipv6 => EthHdr::LEN + Ipv6Hdr::LEN,
        IpFamily::Unknown => 0,
    };

    match protocol {
        IpProto::Tcp => {
            let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, offset)? };
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
            let udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, offset)? };
            unsafe {
                let src_port = u16::from_be_bytes((*udphdr).src);
                let dst_port = u16::from_be_bytes((*udphdr).dst);
                Ok((src_port, dst_port, None))
            }
        }
        _ => Ok((0, 0, None)), // ICMP or other protocols
    }
}
