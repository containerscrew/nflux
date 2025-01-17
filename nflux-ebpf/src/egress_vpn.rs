// // use core::mem;

// // use aya_ebpf::bindings::TC_ACT_PIPE;
// // use aya_ebpf::programs::TcContext;
// // use aya_log_ebpf::info;
// // use network_types::ip::{IpProto, Ipv4Hdr};
// // use network_types::tcp::TcpHdr;
// // use network_types::udp::UdpHdr;
// // use nflux_common::EgressEvent;

// // use crate::maps::{ACTIVE_CONNECTIONS, EGRESS_EVENT};

// // #[inline]
// // fn ptr_at<T>(ctx: &TcContext, offset: usize) -> Result<*const T, ()> {
// //     let start = ctx.data();
// //     let end = ctx.data_end();
// //     let len = mem::size_of::<T>();

// //     if start + offset + len > end {
// //         return Err(());
// //     }

// //     Ok((start + offset) as *const T)
// // }

// pub fn try_tc_egress_vpn(ctx: TcContext) -> Result<i32, ()> {
//     // Parse IPv4 header
//     let ipv4hdr: Ipv4Hdr = match ctx.load(0) {
//         Ok(hdr) => hdr,
//         Err(_) => {
//             info!(&ctx, "Failed to load IPv4 header");
//             return Ok(TC_ACT_PIPE);
//         }
//     };

//     let destination = u32::from_be(ipv4hdr.dst_addr);

//     match ipv4hdr.proto {
//         IpProto::Tcp => {
//             let tcphdr: *const TcpHdr = ptr_at(&ctx, Ipv4Hdr::LEN)?;
//             let src_port = u16::from_be(unsafe { (*tcphdr).source });
//             let dst_port = u16::from_be(unsafe { (*tcphdr).dest });

//             let event = EgressEvent {
//                 dst_ip: destination,
//                 src_port,
//                 dst_port,
//             };

//             EGRESS_EVENT.output(&ctx, &event, 0);

//             // Check if this destination is already active
//             // if unsafe { ACTIVE_CONNECTIONS.get(&destination).is_none() } {
//             //     // Log only new connections
//             //     let event = EgressEvent {
//             //         dst_ip: destination,
//             //         dst_port: dst_port,
//             //     };
//             //
//             //     EGRESS_EVENT.output(&ctx, &event, 0);
//             //
//             //     // Mark connection as active
//             //     if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
//             //         return Err(());
//             //     }
//             //     return Ok(TC_ACT_PIPE)
//             // }
//             Ok(TC_ACT_PIPE)

//             //info!(&ctx, "TCP packet to port {}", dst_port);
//         }
//         IpProto::Udp => {
//             let udphdr: *const UdpHdr = ptr_at(&ctx, Ipv4Hdr::LEN)?;
//             let src_port = u16::from_be(unsafe { (*udphdr).source });
//             let dst_port = u16::from_be(unsafe { (*udphdr).dest });

//             let event = EgressEvent {
//                 dst_ip: destination,
//                 src_port,
//                 dst_port,
//             };
//             EGRESS_EVENT.output(&ctx, &event, 0);

//             Ok(TC_ACT_PIPE)
//             //info!(&ctx, "UDP packet to port {}", dst_port);
//         }
//         _ => {
//             Ok(TC_ACT_PIPE)
//         }
//     }
// }
