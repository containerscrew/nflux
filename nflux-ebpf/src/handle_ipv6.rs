use aya_ebpf::{bindings::TC_ACT_PIPE, programs::TcContext};
use network_types::ip::{IpProto, Ipv6Hdr};
use nflux_common::TcConfig;

pub fn handle_ipv6_packet(
    _ctx: &TcContext,
    _direction: u8,
    _configmap: &TcConfig,
    ipv6hdr: Ipv6Hdr,
    _is_ether: bool,
) -> Result<i32, ()> {
    //let source = u128::from_be(ipv6hdr.src_addr.in6_u.u6_addr32);
    // let destination = u128::from_be(ipv6hdr.dst_addr);
    // let total_len = u16::from_be(ipv6hdr.payload_len);
    let proto = ipv6hdr.next_hdr;

    Ok(TC_ACT_PIPE)
}
