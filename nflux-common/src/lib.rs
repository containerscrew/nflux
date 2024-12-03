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
// pub struct GlobalFirewallRules {
//     pub icmp_enabled: u8,
//     pub allowed_ipv4: [u32; MAX_ALLOWED_IPV4],
//     pub allowed_ports: [u32; MAX_ALLOWED_PORTS],
// }

// #[cfg(feature = "user")]
// pub mod user {
//     use super::*;

//     unsafe impl aya::Pod for GlobalFirewallRules {}
// }

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Ipv4Rule {
    pub action: u8,       // 0 = deny, 1 = allow
    pub ports: [u16; 16], // Up to 16 ports
    pub protocol: u8,     // 6 = TCP, 17 = UDP
}

#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for Ipv4Rule {}
}

// Define the default configuration if the user does not provide one
// pub const DEFAULT_FIREWALL_RULES: GlobalFirewallRules = GlobalFirewallRules {
//     : 0,
//     allowed_ipv4: [0; MAX_ALLOWED_IPV4],
//     allowed_ports: [0; MAX_ALLOWED_PORTS],
// };

pub fn convert_protocol(protocol: u8) -> &'static str {
    match protocol {
        1 => "icmp",
        6 => "tcp",
        17 => "udp",
        _ => "unknown",
    }
}
