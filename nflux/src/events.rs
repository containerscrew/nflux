use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use aya::maps::{MapData, RingBuf};
use nflux_common::{DroppedPacketEvent, IpFamily, TcEvent};
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

pub async fn process_dp_events(
    mut ring_buf: RingBuf<MapData>,
    log_format: String,
) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();

            if data.len() == size_of::<DroppedPacketEvent>() {
                let event: &DroppedPacketEvent =
                    unsafe { &*(data.as_ptr() as *const DroppedPacketEvent) };

                match log_format.as_str() {
                    "json" => {
                        info!(
                            protocol = event.protocol,
                            reason_code = event.reason_code,
                            reason = String::from_utf8_lossy(&event.reason).trim_end_matches('\0'),
                            pid = event.pid,
                            reason_description = String::from_utf8_lossy(&event.reason_description)
                                .trim_end_matches('\0'),
                        );
                    }
                    _ => {
                        info!(
                            "Dropped packet! SkProto: {} SkFamily: {} Reason Code: {} Reason: {:?} PID: {} Human friendly: {:?}",
                            convert_protocol(event.protocol as u8),
                            IpFamily::from_u8(event.family as u8).map(|fam| fam.as_str()).unwrap_or("unknown"),
                            event.reason_code,
                            String::from_utf8_lossy(&event.reason).trim_end_matches('\0'),
                            event.pid,
                            String::from_utf8_lossy(&event.reason_description).trim_end_matches('\0'),
                        );
                    }
                }
            }
        }
        // Avoid busy-loop when no events are arriving
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

pub async fn process_tc_events(
    mut ring_buf: RingBuf<MapData>,
    log_format: String,
    exclude_ports: Option<Vec<u16>>,
) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();

            if data.len() == std::mem::size_of::<TcEvent>() {
                let event: &TcEvent = unsafe { &*(data.as_ptr() as *const TcEvent) };

                if let Some(ref ports) = exclude_ports {
                    if ports.contains(&event.src_port) || ports.contains(&event.dst_port) {
                        continue;
                    }
                }

                let src_ip = to_ipaddr(event.src_ip, event.ip_family.to_owned());
                let dst_ip = to_ipaddr(event.dst_ip, event.ip_family.to_owned());

                let mut msg = format!(
                    "[{}][{}][{}] {}:{} -> {}:{} pkt_len={} ttl={}",
                    convert_direction(event.direction),
                    convert_protocol(event.protocol),
                    event.ip_family.as_str(),
                    src_ip,
                    event.src_port,
                    dst_ip,
                    event.dst_port,
                    event.total_len,
                    event.ttl,
                );

                if convert_protocol(event.protocol) == "tcp" {
                    msg.push_str(&format!(" tcp_flags={}", format_tcp_flags(event.tcp_flags)));
                }

                match log_format.as_str() {
                    "json" => {
                        let tcp_flags_str = format_tcp_flags(event.tcp_flags);
                        info!(
                            dir = %convert_direction(event.direction),
                            ip_family = %event.ip_family.as_str(),
                            protocol = %convert_protocol(event.protocol),
                            pkt_len = event.total_len,
                            ttl = event.ttl,
                            src_ip = %to_ipaddr(event.src_ip, event.ip_family.to_owned()),
                            dst_ip = %to_ipaddr(event.dst_ip, event.ip_family.to_owned()),
                            src_port = event.src_port,
                            dst_port = event.dst_port,
                            tcp_flags = if !tcp_flags_str.is_empty() {
                                Some(tcp_flags_str)
                            } else {
                                None
                            },
                        );
                    }
                    _ => {
                        info!("{}", msg);
                    }
                }
            }
        }

        // Avoid busy-loop when no events are arriving
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mac() {
        let mac = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let formatted_mac = _format_mac(&mac);
        assert_eq!(formatted_mac, "00:1a:2b:3c:4d:5e");
    }

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
