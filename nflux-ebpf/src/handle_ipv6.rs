
// fn start_nflux(ctx: XdpContext) -> Result<u32, ()> {
//     let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

//     match unsafe { (*ethhdr).ether_type } {
//         EtherType::Ipv4 => {
//             let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
//             let source_ip = u32::from_be(unsafe { (*ipv4hdr).src_addr });
//             let dest_ip = u32::from_be(unsafe { (*ipv4hdr).dst_addr });
//             let proto = unsafe { (*ipv4hdr).proto };

//             for prefix_len in (1..=32).rev() {
//                 let key = Key::new(
//                     prefix_len,
//                     LpmKeyIpv4 {
//                         prefix_len,
//                         ip: source_ip & (u32::MAX << (32 - prefix_len)),
//                     },
//                 );

//                 if let Some(rule) = IPV4_RULES.get(&key) {
//                     match proto {
//                         IpProto::Tcp => {
//                             let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
//                             let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
//                             let src_port = u16::from_be(unsafe { (*tcphdr).source });
//                             let syn = unsafe { (*tcphdr).syn() };
//                             let ack = unsafe { (*tcphdr).ack() };

//                             let connection_key = ((source_ip as u64) << 32) | (dest_ip as u64);

//                             // Allow packets for established connections
//                             if CONNECTION_TRACKER.get(&connection_key).is_some() {
//                                 return Ok(xdp_action::XDP_PASS);
//                             }

//                             // Handle new connections (SYN packets)
//                             if syn == 1 && ack == 0 {
//                                 if rule.ports.contains(&dst_port) && rule.action == 1 {
//                                     CONNECTION_TRACKER.insert(&connection_key, &1, 0);
//                                     log_new_connection(ctx, source_ip, dst_port, 6, 1); // Log connection
//                                     return Ok(xdp_action::XDP_PASS);
//                                 } else {
//                                     log_new_connection(ctx, source_ip, dst_port, 6, 0); // Log denied
//                                     return Ok(xdp_action::XDP_DROP);
//                                 }
//                             }

//                             // Default action for other TCP packets
//                             return Ok(xdp_action::XDP_DROP);
//                         }
//                         IpProto::Udp => {
//                             let udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
//                             let dst_port = u16::from_be(unsafe { (*udphdr).dest });

//                             if rule.ports.contains(&dst_port) && rule.action == 1 {
//                                 log_new_connection(ctx, source_ip, dst_port, 17, 1);
//                                 return Ok(xdp_action::XDP_PASS);
//                             }
//                             return Ok(xdp_action::XDP_DROP);
//                         }
//                         IpProto::Icmp => {
//                             if let Some(&icmp_ping) = ICMP_RULE.get(0) {
//                                 if icmp_ping == 1 {
//                                     log_new_connection(ctx, source_ip, 0, 1, 1);
//                                     return Ok(xdp_action::XDP_PASS);
//                                 }
//                             }
//                             return Ok(xdp_action::XDP_DROP);
//                         }
//                         _ => return Ok(xdp_action::XDP_DROP),
//                     }
//                 }
//             }
//             Ok(xdp_action::XDP_DROP)
//         }
//         EtherType::Ipv6 => {
//             let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
//             let proto = unsafe { (*ipv6hdr).next_hdr };
//             let source_ip = unsafe { (*ipv6hdr).src_addr.in6_u.u6_addr8 };
//             let dest_ip = unsafe { (*ipv6hdr).dst_addr.in6_u.u6_addr8 };

//             let connection_key = [source_ip, dest_ip];

//             for prefix_len in (1..=128).rev() {
//                 let key = Key::new(
//                     prefix_len,
//                     LpmKeyIpv6 {
//                         prefix_len,
//                         ip: source_ip,
//                     },
//                 );

//                 if let Some(rule) = IPV6_RULES.get(&key) {
//                     match proto {
//                         IpProto::Tcp => {
//                             let tcphdr: *const TcpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)? };
//                             let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
//                             let syn = unsafe { (*tcphdr).syn() };
//                             let ack = unsafe { (*tcphdr).ack() };

//                             // Allow packets for established connections
//                             if CONNECTION_TRACKER.get(&connection_key).is_some() {
//                                 return Ok(xdp_action::XDP_PASS);
//                             }

//                             // Handle new connections (SYN packets)
//                             if syn == 1 && ack == 0 {
//                                 if rule.ports.contains(&dst_port) && rule.action == 1 {
//                                     CONNECTION_TRACKER.insert(&connection_key, &1, 0);
//                                     log_new_connection(ctx, 0, dst_port, 6, 1);
//                                     return Ok(xdp_action::XDP_PASS);
//                                 } else {
//                                     log_new_connection(ctx, 0, dst_port, 6, 0);
//                                     return Ok(xdp_action::XDP_DROP);
//                                 }
//                             }

//                             // Default action for other TCP packets
//                             return Ok(xdp_action::XDP_DROP);
//                         }
//                         IpProto::Udp => {
//                             let udphdr: *const UdpHdr = unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv6Hdr::LEN)? };
//                             let dst_port = u16::from_be(unsafe { (*udphdr).dest });

//                             if rule.ports.contains(&dst_port) && rule.action == 1 {
//                                 log_new_connection(ctx, 0, dst_port, 17, 1);
//                                 return Ok(xdp_action::XDP_PASS);
//                             }
//                             return Ok(xdp_action::XDP_DROP);
//                         }
//                         IpProto::Icmp => {
//                             if let Some(&icmp_ping) = ICMP_RULE.get(0) {
//                                 if icmp_ping == 1 {
//                                     log_new_connection(ctx, 0, 0, 1, 1);
//                                     return Ok(xdp_action::XDP_PASS);
//                                 }
//                             }
//                             return Ok(xdp_action::XDP_DROP);
//                         }
//                         _ => return Ok(xdp_action::XDP_DROP),
//                     }
//                 }
//             }
//             Ok(xdp_action::XDP_DROP)
//         }
//         _ => Ok(xdp_action::XDP_DROP),
//     }
// }
