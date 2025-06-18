use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use aya::maps::{MapData, RingBuf};
use nflux_common::TcEvent;
use tokio::sync::watch;
use tracing::info;

use crate::utils::{convert_direction, convert_protocol};

fn _format_mac(mac: &[u8; 6]) -> String {
    mac.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(":")
}

fn to_ipaddr(ip: [u8; 16], ip_family: u8) -> IpAddr {
    match ip_family {
        4 => IpAddr::V4(Ipv4Addr::new(ip[12], ip[13], ip[14], ip[15])),
        6 => IpAddr::V6(Ipv6Addr::from(ip)),
        _ => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
    }
}

pub async fn process_event(
    mut ring_buf: RingBuf<MapData>,
    log_format: String,
    exclude_ports: Option<Vec<u16>>,
    mut shutdown: watch::Receiver<bool>,
) -> Result<(), anyhow::Error> {
    loop {
        if *shutdown.borrow() {
            break;
        }

        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();

            if data.len() == std::mem::size_of::<TcEvent>() {
                let event: &TcEvent = unsafe { &*(data.as_ptr() as *const TcEvent) };

                if let Some(ref ports) = exclude_ports {
                    if ports.contains(&event.src_port) || ports.contains(&event.dst_port) {
                        continue;
                    }
                }

                let mut msg = format!(
                    "[{}][{}][{}] {}:{} -> {}:{} pkt_len={} ttl={}",
                    convert_direction(event.direction),
                    convert_protocol(event.protocol),
                    event.ip_family.as_str(),
                    to_ipaddr(event.src_ip, event.ip_family.to_owned()),
                    event.src_port,
                    to_ipaddr(event.dst_ip, event.ip_family.to_owned()),
                    event.dst_port,
                    event.total_len,
                    event.ttl,
                );
                
                if convert_protocol(event.protocol) == "tcp" {
                    msg.push_str(&format!(" tcp_flags={}", event.tcp_flags));
                }
                
                match log_format.as_str() {
                    "json" => {
                        info!(
                            dir = %convert_direction(event.direction),
                            ip_family = %event.ip_family.as_str(),
                            protocol = %convert_protocol(event.protocol),
                            pkt_len = event.total_len,
                            ttl = event.ttl,
                            src_ip = %to_ipaddr(event.src_ip, event.ip_family.to_owned()),
                            dst_ip = %to_ipaddr(event.dst_ip, 4),
                            src_port = event.src_port,
                            dst_port = event.dst_port,
                        );
                    }
                    _ => {
                        info!("{}", msg);
                    }
                }
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {},
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    break;
                }
            }
        }
    }

    Ok(())
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

        let result = to_ipaddr(ip, 4);
        assert_eq!(result, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
    }

    #[test]
    fn test_to_ipaddr_ipv6() {
        let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0x0202, 0xb3ff, 0xfe1e, 0x8329).octets();
        let result = to_ipaddr(ip, 6);
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
