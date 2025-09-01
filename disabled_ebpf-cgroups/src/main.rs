#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{cgroup_skb, map},
    maps::RingBuf,
    programs::SkBuffContext,
};
use aya_log_ebpf::{info, warn};
use network_types::ip::Ipv4Hdr;
use nflux_common::NetworkEvent;

#[map]
pub static CGROUP_NETWORK_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;

#[cgroup_skb]
pub fn cgroups_traffic_egress(ctx: SkBuffContext) -> i32 {
    match try_cgroups_traffic(ctx, 1) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[cgroup_skb]
pub fn cgroups_traffic_ingress(ctx: SkBuffContext) -> i32 {
    match try_cgroups_traffic(ctx, 0) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_cgroups_traffic(
    ctx: SkBuffContext,
    direction: u8,
) -> Result<i32, i32> {
    let protocol = unsafe { (*ctx.skb.skb).protocol } as u16;
    let eth_proto = u16::from_be(protocol);

    match eth_proto {
        ETH_P_IP => {
            // Handle IPv4 packets
            // Another way to get the protocol field from the IPv4 header
            // Load only the 'proto' field from the IPv4 header using its byte offset.
            // Useful when you want a specific field without reading the entire struct.
            // let prot = ctx.load::<IpProto>(offset_of!(Ipv4Hdr, proto)).map_err(|_| 0)?;

            // In cgroup_skb programs, especially in container environments (e.g., Podman, Docker),
            // the Ethernet header is typically stripped by the time the packet reaches this hook.
            // This is because virtual interfaces (like veth) used by containers pass the packet
            // further into the networking stack, where layer 2 (Ethernet) data is no longer
            // present. Therefore, we start reading directly from offset 0, which
            // corresponds to the IP header.
            let ipv4_hdr = ctx.load::<Ipv4Hdr>(0).map_err(|_| {
                warn!(&ctx, "Error loading Ipv4Hdr");
                0
            })?;

            let mut src_ip = [0u8; 16];
            src_ip[12..].copy_from_slice(&ipv4_hdr.src_addr);
            let mut dst_ip = [0u8; 16];
            dst_ip[12..].copy_from_slice(&ipv4_hdr.dst_addr);

            unsafe {
                if let Some(mut data) = CGROUP_NETWORK_EVENT.reserve::<NetworkEvent>(0) {
                    let ptr = data.as_mut_ptr();
                    core::ptr::write(
                        ptr,
                        NetworkEvent {
                            src_ip,
                            dst_ip,
                            total_len: ipv4_hdr.total_len(),
                            ttl: ipv4_hdr.ttl,
                            src_port: 0,
                            dst_port: 0,
                            protocol: ipv4_hdr.proto as u8,
                            direction, // 0 for ingress, 1 for egress
                            ip_family: nflux_common::IpFamily::Ipv4,
                            tcp_flags: None, // No TCP flags for non-TCP packets
                        },
                    );
                    data.submit(0);
                }
            }
        }
        ETH_P_IPV6 => {
            // Handle IPv6 packets
            //info!(&ctx, "IPv6 packet detected");

            // let ip_hdr = ctx.load::<Ipv6Hdr>(0).map_err(|_| {
            //     warn!(&ctx, "Error loading Ipv6Hdr");
            //     0
            // })?;

            // let network_event = NetworkEventIpv6 {
            //     src_addr: ip_hdr.src_addr().to_bits(),
            //     dst_addr: ip_hdr.dst_addr().to_bits(),
            //     protocol: ip_hdr.next_hdr as u8,
            // };

            // if let Some(mut data) = NETWORK_EVENT_IPV6.reserve::<NetworkEventIpv6>(0) {
            //     unsafe { *data.as_mut_ptr() = network_event }
            //     data.submit(0);
            // }
        }
        _ => {
            info!(&ctx, "Unknown protocol {}", protocol);
            return Ok(1);
        }
    }

    // Allow the packet to pass through
    Ok(1)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
