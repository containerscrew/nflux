use std::{net::Ipv4Addr};

use aya::maps::{MapData, RingBuf};
use nflux_common::TcEvent;
use tracing::info;

use crate::utils::{convert_direction, convert_protocol};

fn format_mac(mac: &[u8; 6]) -> String {
    mac.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(":")
}

pub async fn process_event(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            // Get the data from the event
            let data = event.as_ref();

            // Make sure the data is the correct size
            if data.len() == std::mem::size_of::<TcEvent>() {
                let event: &TcEvent = unsafe { &*(data.as_ptr() as *const TcEvent) };

                let log_format = "text" ;

                match log_format {
                    "json" => {
                        info!(
                            dir = %convert_direction(event.direction),
                            ip_family = %event.ip_family.as_str(),
                            protocol = %convert_protocol(event.protocol),
                            total_len = event.total_len,
                            ttl = event.ttl,
                            src_ip = %Ipv4Addr::from(event.src_ip),
                            dst_ip = %Ipv4Addr::from(event.dst_ip),
                            src_port = event.src_port,
                            dst_port = event.dst_port,
                            src_mac = %format_mac(&event.src_mac),
                            dst_mac = %format_mac(&event.dst_mac),
                        );
                    },
                    _ => {
                        // Default log format (text format)
                        info!(
                            "[{}][{}][{}] {}:{} -> {}:{} len={} ttl={}",
                            convert_direction(event.direction),
                            convert_protocol(event.protocol),
                            event.ip_family.as_str(),
                            Ipv4Addr::from(event.src_ip),
                            event.src_port,
                            Ipv4Addr::from(event.dst_ip),
                            event.dst_port,
                            event.total_len,
                            event.ttl,
                            //    format_mac(&event.src_mac),
                            //    format_mac(&event.dst_mac)
                        );
                    }
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mac() {
        let mac = [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        let formatted_mac = format_mac(&mac);
        assert_eq!(formatted_mac, "00:1a:2b:3c:4d:5e");
    }
}
