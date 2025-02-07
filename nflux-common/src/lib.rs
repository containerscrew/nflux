#![no_std]
pub mod utils;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcConfig {
    pub disable_egress: u8, // 0 = no, 1 = yes
    pub disable_ingress: u32, // 0 = no, 1 = yes
    pub disable_private_ips: u8,  // 0 = no, 1 = yes
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcEvent {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub direction: u8, // 0: ingress, 1: egress
    pub pid: u32,
}
#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for TcConfig {}
}

pub fn convert_protocol(protocol: u8) -> &'static str {
    match protocol {
        1 => "icmp",
        6 => "tcp",
        17 => "udp",
        _ => "unknown",
    }
}
