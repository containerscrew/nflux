#![no_std]

pub const MAX_FIREWALL_RULES: u32 = 32;
pub const MAX_RULES_PORT: usize = 32;
pub const MAX_ALLOWED_PORTS: usize = 1024;
pub const MAX_ALLOWED_IPV4: usize = 1024;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectionEvent {
    pub src_addr: u32,
    pub dst_port: u16,
    pub protocol: u8, // 6 for TCP, 17 for UDP
}

// #[repr(C)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct AppConfig {
//     pub log_interval_secs: u64,
//     pub icmp_enabled: u32,
//     pub allowed_ipv4: [u32; MAX_ALLOWED_IPV4],
//     pub allowed_ports: [u32; MAX_ALLOWED_PORTS],
// }

pub fn convert_protocol(protocol: u8) -> &'static str {
    match protocol {
        1 => "icmp",
        6 => "tcp",
        17 => "udp",
        _ => "unknown",
    }
}
