use core::mem;

use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::helpers::gen::bpf_get_current_pid_tgid;
use aya_ebpf::programs::TcContext;
use aya_log_ebpf::info;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use network_types::tcp::TcpHdr;
use network_types::udp::UdpHdr;
use nflux_common::EgressEvent;

use crate::maps::{ACTIVE_CONNECTIONS, EGRESS_CONFIG, EGRESS_EVENT};

#[inline]
fn ptr_at<T>(ctx: &TcContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[inline]
unsafe fn log_connection(ctx: &TcContext, destination: u32, src_port: u16, dst_port: u16, protocol: u8, pid: u64) {
    // Check if this destination is already active
    if ACTIVE_CONNECTIONS.get(&destination).is_none() {
        // Log only new connections
        let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
        EGRESS_EVENT.output(ctx, &event, 0);

        // Mark connection as active
        if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
            return;
        }
    }
}

pub fn try_tc_egress(ctx: TcContext) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;

    let egress_config = EGRESS_CONFIG.get(0).ok_or(())?;

    // if let Some(&egress_config) = EGRESS_CONFIG.get(0) {
    //     let egress_config = egres
    //     if egress_config.log_udp == 0 {
    //         info!(&ctx, "log_udp is disabled");
    //     }
    // }

    match ethhdr.ether_type {
        EtherType::Ipv4 => unsafe {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            let destination = u32::from_be(ipv4hdr.dst_addr);

            match ipv4hdr.proto {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                    let src_port = u16::from_be((*tcphdr).source);
                    let dst_port = u16::from_be((*tcphdr).dest);
                    let protocol = IpProto::Tcp as u8;
                    let pid_tgid = bpf_get_current_pid_tgid();
                    let pid = pid_tgid >> 32;

                    // If log_tcp_connections is enabled, log the connection
                    if egress_config.log_tcp_connections == 1 {
                        log_connection(&ctx, destination, src_port, dst_port, protocol, pid);
                        // If log_only_new_connections is enabled, only log new connections
                        // match egress_config.log_only_new_connections {
                        //     0 => {
                        //         // Check if this destination is already active
                        //         if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                        //             // Log only new connections
                        //             let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
                        //             EGRESS_EVENT.output(&ctx, &event, 0);

                        //             // Mark connection as active
                        //             if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
                        //                 return Err(());
                        //             }
                        //         }
                        //     }
                        //     1 => {
                        //         // Log all connections
                        //         let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
                        //         EGRESS_EVENT.output(&ctx, &event, 0);
                        //     }
                        //     _ => {}
                        // }
                    }
                    return Ok(TC_ACT_PIPE)
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr = ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN)?;
                    let src_port = u16::from_be((*udphdr).source);
                    let dst_port = u16::from_be((*udphdr).dest);
                    let protocol = IpProto::Udp as u8;
                    let pid_tgid = bpf_get_current_pid_tgid();
                    let pid = pid_tgid >> 32;

                    // If log_tcp_connections is enabled, log the connection
                    if egress_config.log_udp_connections == 1 {
                        // If log_only_new_connections is enabled, only log new connections
                        match egress_config.log_only_new_connections {
                            0 => {
                                // Check if this destination is already active
                                if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                                    // Log only new connections
                                    let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
                                    EGRESS_EVENT.output(&ctx, &event, 0);

                                    // Mark connection as active
                                    if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
                                        return Err(());
                                    }
                                }
                            }
                            1 => {
                                // Log all connections
                                let event = EgressEvent { dst_ip: destination, src_port, dst_port, protocol, pid };
                                EGRESS_EVENT.output(&ctx, &event, 0);
                            }
                            _ => {}
                        }
                    }

                    return Ok(TC_ACT_PIPE)
                }
                _ => {}
            }
        }
        EtherType::Ipv6 => {
            info!(&ctx, "is an ipv6 packet!");
            //let ipv6hdr: *const Ipv6Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            //let proto = unsafe { (*ipv6hdr).next_hdr };
            //let destination = unsafe { (*ipv6hdr).dst_addr.in6_u.u6_addr8 };

            // Create here a fake event
            // IPV6 is not implemented yet
            let event = EgressEvent { dst_ip: u32::from_be_bytes([192, 67, 4, 2]), src_port: 111, dst_port: 99, protocol: 6, pid: 1234};
            EGRESS_EVENT.output(&ctx, &event, 0);
            return Ok(TC_ACT_PIPE)
        }
        EtherType::FibreChannel => {
            info!(&ctx, "ether type fibrechannel!!");
            return Ok(TC_ACT_PIPE)
        }
        EtherType::Arp => {
            info!(&ctx, "ARP packet!!");
            return Ok(TC_ACT_PIPE)
        }
        _ => {
            //info!(&ctx, "other ether type!");
            return Ok(TC_ACT_PIPE)
        },
    }

    Ok(TC_ACT_PIPE)
}
