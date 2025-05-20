use std::net::Ipv4Addr;

use aya::maps::{MapData, RingBuf};
use nflux_common::TcEvent;
use tracing::info;

use crate::utils::{convert_protocol};

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

                let direction = if event.direction == 0 {
                    "ingress"
                } else {
                    "egress"
                };

                info!(
                    "dir={} type={} protocol={} total_len={}B ttl={} src_ip={} dst_ip={} src_port={} dst_port={} src_mac={} dst_mac={}",
                    direction,
                    event.ip_family.as_str(),
                    convert_protocol(event.protocol),
                    event.total_len,
                    event.ttl,
                    Ipv4Addr::from(event.src_ip),
                    Ipv4Addr::from(event.dst_ip),
                    event.src_port,
                    event.dst_port,
                    format_mac(&event.src_mac),
                    format_mac(&event.dst_mac)
                );
            }
        }

        // Sleep for a while
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
