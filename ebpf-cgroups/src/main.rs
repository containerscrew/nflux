#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{cgroup_skb, map},
    maps::RingBuf,
    programs::SkBuffContext,
};
use aya_log_ebpf::{info, warn};
use network_types::ip::Ipv4Hdr;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkEventIpv4 {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub protocol: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetworkEventIpv6 {
    pub src_addr: u128,
    pub dst_addr: u128,
    pub protocol: u8,
}

#[map]
pub static NETWORK_EVENT_IPV4: RingBuf = RingBuf::with_byte_size(4096, 0);

// #[map]
// pub static NETWORK_EVENT_IPV6: RingBuf = RingBuf::with_byte_size(4096, 0);

const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86DD;

#[cgroup_skb]
pub fn csp(ctx: SkBuffContext) -> i32 {
    match try_csp(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_csp(ctx: SkBuffContext) -> Result<i32, i32> {
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
            let ip_hdr = ctx.load::<Ipv4Hdr>(0).map_err(|_| {
                warn!(&ctx, "Error loading Ipv4Hdr");
                0
            })?;

            let network_event = NetworkEventIpv4 {
                src_addr: ip_hdr.src_addr().to_bits(),
                dst_addr: ip_hdr.dst_addr().to_bits(),
                protocol: ip_hdr.proto as u8,
            };

            if let Some(mut data) = NETWORK_EVENT_IPV4.reserve::<NetworkEventIpv4>(0) {
                unsafe { *data.as_mut_ptr() = network_event }
                data.submit(0);
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
