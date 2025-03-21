#![no_std]

pub mod utils;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Configmap {
    pub disable_private_ips: u8, // 0 = no, 1 = yes
    pub enable_udp: u8,          // 0 = no, 1 = yes
    pub enable_icmp: u8,         // 0 = no, 1 = yes
    pub enable_tcp: u8,          // 0 = no, 1 = yes
    pub log_interval: u8,        // Log connection of same ip --> port every X seconds
    pub full_log: u8,            // Log every packet
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpType {
    Ipv4,
    Ipv6,
}

impl IpType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IpType::Ipv4 => "IPv4",
            IpType::Ipv6 => "IPv6",
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcEvent {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub total_len: u16,
    pub ttl: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub direction: u8, // 0: ingress, 1: egress
    pub ip_type: IpType,
    pub pid: u32,
}

#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for Configmap {}
}
