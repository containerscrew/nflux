#![no_std]

pub mod utils;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Configmap {
    pub disable_private_ips: u8, // 0 = no, 1 = yes
    pub disable_udp: u8,         // 0 = no, 1 = yes
    pub disable_icmp: u8,        // 0 = no, 1 = yes
    pub disable_tcp: u8,         // 0 = no, 1 = yes
    pub log_interval: u8,        // Log connection of same ip --> port every X seconds
    pub disable_full_log: u8,    // Disable full packet log
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
}

pub const MAX_BUF_SIZE: usize = 2048;
pub const TASK_COMM_LEN: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Read,
    Write,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TLSData {
    pub kind: Kind,
    pub len: i32,
    pub buf: [u8; MAX_BUF_SIZE],
    pub comm: [u8; TASK_COMM_LEN],
}

#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for Configmap {}
}
