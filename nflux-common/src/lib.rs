#![no_std]

pub const MAX_FIREWALL_RULES: u32 = 32;
pub const MAX_RULES_PORT: usize = 32;
pub const MAX_ALLOWED_PORTS: usize = 1024;
pub const MAX_ALLOWED_IPV4: usize = 1024;

pub mod utils;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EgressConfig {
    pub log_only_new_connections: u8, // 0 = no, 1 = yes
    pub log_refresh_new_connections_every: u32,
    pub log_udp_connections: u8,  // 0 = no, 1 = yes
    pub log_tcp_connections: u8,  // 0 = no, 1 = yes
    pub log_icmp_connections: u8, // 0 = no, 1 = yes
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectionEvent {
    pub src_addr: u32,
    pub dst_port: u16,
    pub protocol: u8, // 6 for TCP, 17 for UDP, 1 for ICMP
    pub action: u8,   // 0 for deny, 1 for allow
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EgressEvent {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
    pub direction: u8, // 0: ingress, 1: egress
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IpRule {
    pub action: u8,       // 0 = deny, 1 = allow
    pub ports: [u16; 16], // Up to 16 ports
    pub protocol: u8,     // 6 = TCP, 17 = UDP, 1 = ICMP
    pub priority: u32,    // Lower number means higher priority
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LpmKeyIpv4 {
    pub prefix_len: u32,
    pub ip: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LpmKeyIpv6 {
    pub prefix_len: u32,
    pub ip: [u8; 16],
}

#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for IpRule {}
    unsafe impl aya::Pod for LpmKeyIpv4 {}
    unsafe impl aya::Pod for LpmKeyIpv6 {}
    unsafe impl aya::Pod for EgressConfig {}
}

pub fn convert_protocol(protocol: u8) -> &'static str {
    match protocol {
        1 => "icmp",
        6 => "tcp",
        17 => "udp",
        _ => "unknown",
    }
}
