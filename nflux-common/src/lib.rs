#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

pub mod utils;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Configmap {
    pub disable_private_ips: u8, // 0 = no, 1 = yes
    pub disable_udp: u8,         // 0 = no, 1 = yes
    pub disable_icmp: u8,        // 0 = no, 1 = yes
    pub disable_tcp: u8,         // 0 = no, 1 = yes
    pub log_interval: u64,       // Log connection of same ip --> port every X seconds
    pub disable_full_log: u8,    // Disable full packet log
    pub listen_port: u16,        // Filter port to sniff.
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpFamily {
    Ipv4,
    Ipv6,
}

impl IpFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            IpFamily::Ipv4 => "IPv4",
            IpFamily::Ipv6 => "IPv6",
        }
    }

    pub fn to_owned(&self) -> u8 {
        match self {
            IpFamily::Ipv4 => 4,
            IpFamily::Ipv6 => 6,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TcEvent {
    pub src_ip: [u8; 16],
    pub dst_ip: [u8; 16],
    pub total_len: u16,
    pub ttl: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub direction: u8, // 0: ingress, 1: egress
    pub ip_family: IpFamily,
    pub tcp_flags: TcpFlags,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TcpFlags {
    pub syn: u8,
    pub ack: u8,
    pub fin: u8,
    pub rst: u8,
}

impl fmt::Display for TcpFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        if self.syn != 0 {
            parts.push("SYN");
        }
        if self.ack != 0 {
            parts.push("ACK");
        }
        if self.fin != 0 {
            parts.push("FIN");
        }
        if self.rst != 0 {
            parts.push("RST");
        }

        write!(f, "{}", parts.join(","))
    }
}

// #[repr(C)]
// pub struct ArpEvent {
//     pub op: u16,              // Request o reply
//     pub sender_ip: [u8; 4],
//     pub target_ip: [u8; 4],
//     pub sender_mac: [u8; 6],
//     pub target_mac: [u8; 6],
// }

pub const MAX_BUF_SIZE: usize = 2048;
pub const TASK_COMM_LEN: usize = 16;

#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for Configmap {}
}
