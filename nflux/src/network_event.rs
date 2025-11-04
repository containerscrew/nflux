use std::{
    fmt::{self},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use aya::maps::{MapData, RingBuf};
use nflux_common::dto::{ArpEvent, NetworkEvent, TcpFlags};
use tracing::{info, warn};

use crate::utils::{convert_direction, convert_protocol, format_tcp_flags};

fn _format_mac(mac: &[u8; 6]) -> String {
    mac.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(":")
}

fn to_ipaddr(
    ip: [u8; 16],
    ip_family: u8,
) -> IpAddr {
    match ip_family {
        2 => IpAddr::V4(Ipv4Addr::new(ip[12], ip[13], ip[14], ip[15])), // AF_INET
        10 => IpAddr::V6(Ipv6Addr::from(ip)),                           // AF_INET6
        _ => {
            warn!("Unknown ip_family: {}", ip_family);
            IpAddr::V4(Ipv4Addr::UNSPECIFIED)
        }
    }
}

fn _ip_familiy_as_str(ip_family: u8) -> &'static str {
    match ip_family {
        2 => "IPv4",
        10 => "IPv6",
        _ => "Unknown",
    }
}

pub async fn process_networking_event(
    mut ring_buf: RingBuf<MapData>,
    log_format: String,
    exclude_ports: Option<Vec<u16>>,
) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            let data: &[u8] = event.as_ref();

            if data.len() == size_of::<NetworkEvent>() {
                let event: &NetworkEvent = unsafe { &*(data.as_ptr() as *const NetworkEvent) };

                if let Some(ref ports) = exclude_ports {
                    if ports.contains(&event.src_port) || ports.contains(&event.dst_port) {
                        continue;
                    }
                }
                match log_format.as_str() {
                    "json" => {
                        let e = event;
                        info!(
                            direction = %convert_direction(e.direction),
                            protocol = %convert_protocol(e.protocol),
                            ip_family = %e.ip_family.as_str(),
                            src_ip = %to_ipaddr(e.src_ip, e.ip_family.to_owned()),
                            src_port = e.src_port,
                            dst_ip = %to_ipaddr(e.dst_ip, e.ip_family.to_owned()),
                            dst_port = e.dst_port,
                            total_len = e.total_len,
                            ttl = e.ttl,
                            tcp_flags = if let Some(flags) = e.tcp_flags {
                                format_tcp_flags(flags)
                            } else {
                                "".to_string()
                            },
                        );
                    }
                    _ => {
                        info!("{}", DisplayNetworkEvent(*event));
                    }
                }
            }
        }

        // Avoid busy-loop when no events are arriving
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

pub async fn process_arp_events(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            let data: &[u8] = event.as_ref();
            if data.len() == std::mem::size_of::<ArpEvent>() {
                let event: &ArpEvent = unsafe { &*(data.as_ptr() as *const ArpEvent) };
                info!(
                    message = "arp event",
                    op_code = event.arp_op_to_str(),
                    ip_family = event.ip_family.as_str(),
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_format_mac() {
    //     let mac = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
    //     let formatted_mac = format_mac(&mac);
    //     assert_eq!(formatted_mac, "00:1a:2b:3c:4d:5e");
    // }

    #[test]
    fn test_to_ipaddr_ipv4() {
        let mut ip = [0u8; 16];
        ip[10] = 0xFF;
        ip[11] = 0xFF;
        ip[12] = 192;
        ip[13] = 168;
        ip[14] = 1;
        ip[15] = 100;

        let result = to_ipaddr(ip, 2);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
    }

    #[test]
    fn test_to_ipaddr_ipv6() {
        let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0x0202, 0xb3ff, 0xfe1e, 0x8329).octets();
        let result = to_ipaddr(ip, 10);
        assert_eq!(
            result,
            IpAddr::V6(Ipv6Addr::new(
                0xfe80, 0, 0, 0, 0x0202, 0xb3ff, 0xfe1e, 0x8329
            ))
        );
    }

    #[test]
    fn test_to_ipaddr_invalid_family() {
        let result = to_ipaddr([0u8; 16], 123);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    }
}

// DisplayTcpFlags is a helper struct to format TCP flags
pub struct DisplayTcpFlags(pub TcpFlags);
impl fmt::Display for DisplayTcpFlags {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let flags = &self.0;

        let mut parts = Vec::new();

        if flags.syn != 0 {
            parts.push("SYN");
        }
        if flags.ack != 0 {
            parts.push("ACK");
        }
        if flags.fin != 0 {
            parts.push("FIN");
        }
        if flags.rst != 0 {
            parts.push("RST");
        }
        if flags.psh != 0 {
            parts.push("PSH");
        }
        if flags.urg != 0 {
            parts.push("URG");
        }
        if flags.ece != 0 {
            parts.push("ECE");
        }
        if flags.cwr != 0 {
            parts.push("CWR");
        }

        if parts.is_empty() {
            write!(f, "none")
        } else {
            write!(f, "{}", parts.join(","))
        }
    }
}

// Supertrait to convert NetworkEvent to a Displayable format
pub struct DisplayNetworkEvent(pub NetworkEvent);

impl fmt::Display for DisplayNetworkEvent {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let event = &self.0;
        write!(
            f,
            "[{}][{}][{}] {}:{} -> {}:{} -> data_len={} -> ttl={}",
            convert_direction(event.direction),
            convert_protocol(event.protocol),
            event.ip_family.as_str(),
            to_ipaddr(event.src_ip, event.ip_family.to_owned()),
            event.src_port,
            to_ipaddr(event.dst_ip, event.ip_family.to_owned()),
            event.dst_port,
            event.total_len,
            event.ttl,
        )?;

        if let Some(flags) = event.tcp_flags {
            write!(f, ", tcp_flags={}", DisplayTcpFlags(flags))?;
        }

        write!(f, "")
    }
}
