#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IpFamily {
    Ipv4,
    Ipv6,
    Unknown,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TcpFlags {
    pub syn: u8,
    pub ack: u8,
    pub fin: u8,
    pub rst: u8,
    pub psh: u8,
    pub urg: u8,
    pub ece: u8,
    pub cwr: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Configmap {
    pub disable_udp: u8,      // 0 = no, 1 = yes
    pub disable_icmp: u8,     // 0 = no, 1 = yes
    pub disable_tcp: u8,      // 0 = no, 1 = yes
    pub disable_arp: u8,      // 0 = no, 1 = yes
    pub log_interval: u64,    // Log connection of same ip --> port every X seconds
    pub disable_full_log: u8, // Disable full packet log
    pub listen_port: u16,     // Filter port to sniff.
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkEvent {
    pub src_ip: [u8; 16],
    pub dst_ip: [u8; 16],
    pub total_len: u16,
    pub ttl: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,  // 1: ICMP, 6: TCP, 17: UDP
    pub direction: u8, // 0: ingress, 1: egress
    pub ip_family: IpFamily,
    pub tcp_flags: Option<TcpFlags>,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArpEvent {
    pub op_code: u16,
    pub ip_family: IpFamily,
    pub sha: [u8; 6],
    pub spa: [u8; 16],
    pub tha: [u8; 6],
    pub tpa: [u8; 16],
}

impl ArpEvent {
    pub fn arp_op_to_str(&self) -> &'static str {
        match self.op_code {
            1 => "request",
            2 => "reply",
            3 => "request reverse",
            4 => "reply reverse",
            _ => "unknown",
        }
    }
}

#[cfg(feature = "user")]
pub mod user {
    use super::*;

    unsafe impl aya::Pod for Configmap {}
}
