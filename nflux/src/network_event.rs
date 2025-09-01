use std::fmt::{self, Display};

use aya::maps::{MapData, RingBuf};
use nflux_common::dto::NetworkEvent;
use tracing::{info, warn};

use crate::utils::{convert_direction, convert_protocol, to_ipaddr};

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
        )?;

        if let Some(flags) = event.tcp_flags {
            write!(f, ", tcp_flags: {:?}", flags)?;
        }

        write!(f, "")
    }
}

pub async fn process_ring_buffer<T>(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error>
where
    T: Display,
{
    loop {
        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();

            if data.len() == std::mem::size_of::<T>() {
                let event: &T = unsafe { &*(data.as_ptr() as *const T) };
                info!("{}", event);
            } else {
                warn!(
                    "Event size mismatch: expected {}, got {}",
                    std::mem::size_of::<T>(),
                    data.len()
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
