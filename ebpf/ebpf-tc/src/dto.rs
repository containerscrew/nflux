use network_types::ip::{Ipv4Hdr, Ipv6Hdr};

pub enum IpHeader {
    V4(Ipv4Hdr),
    V6(Ipv6Hdr),
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ActiveConnectionKey {
    pub protocol: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub src_ip: [u8; 16],
    pub dst_ip: [u8; 16],
}
