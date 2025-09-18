use std::fmt::{self};

use nflux_common::dto::{NetworkEvent, TcpFlags};

use crate::utils::{convert_direction, convert_protocol, to_ipaddr};

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
            write!(f, ", tcp_flags={}", DisplayTcpFlags(flags))?;
        }

        write!(f, "")
    }
}

// pub async fn process_ring_buffer<T>(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error>
// where
//     T: Display,
// {
//     loop {
//         while let Some(event) = ring_buf.next() {
//             let data = event.as_ref();

//             if data.len() == std::mem::size_of::<T>() {
//                 let event: &T = unsafe { &*(data.as_ptr() as *const T) };
//                 info!("{}", event);
//             } else {
//                 warn!(
//                     "Event size mismatch: expected {}, got {}",
//                     std::mem::size_of::<T>(),
//                     data.len()
//                 );
//             }
//         }
//         tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//     }
// }
