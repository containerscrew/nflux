#![no_std]

pub mod dto;
pub mod utils;

// #[repr(C)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct TcEvent {
//     pub src_ip: [u8; 16],
//     pub dst_ip: [u8; 16],
//     pub total_len: u16,
//     pub ttl: u8,
//     pub src_port: u16,
//     pub dst_port: u16,
//     pub protocol: u8,
//     pub direction: u8, // 0: ingress, 1: egress
//     pub ip_family: IpFamily,
//     pub tcp_flags: TcpFlags,
// }

// #[repr(C)]
// pub struct ArpEvent {
//     pub op: u16,              // Request o reply
//     pub sender_ip: [u8; 4],
//     pub target_ip: [u8; 4],
//     pub sender_mac: [u8; 6],
//     pub target_mac: [u8; 6],
// }
