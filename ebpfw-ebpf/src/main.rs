#![no_std]
#![no_main]

use core::mem;

use aya_ebpf::{
    bindings::{TC_ACT_PIPE, TC_ACT_SHOT},
    macros::{classifier, map},
    maps::HashMap,
    programs::TcContext,
};
use aya_log_ebpf::info;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
};

#[map]
static BLOCKLIST: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);

#[classifier]
pub fn ebpfw(ctx: TcContext) -> i32 {
    match start_ebpfw(ctx) {
        Ok(ret) => ret,
        Err(_) => TC_ACT_SHOT,
    }
}

fn block_ip(address: u32) -> bool {
    // Check if the IP address is in the BLOCKLIST map. If it is, return true to indicate it should be blocked.
    unsafe { BLOCKLIST.get(&address).is_some() }
}

#[no_mangle]
static TRAFFIC_DIRECTION: i32 = 0;

#[inline]
fn is_ingress() -> bool {
    // Use the TRAFFIC_DIRECTION to determine if the traffic is ingress.
    let traffic_direction = unsafe { core::ptr::read_volatile(&TRAFFIC_DIRECTION) };
    traffic_direction == -1
}

#[inline]
fn ptr_at<T>(ctx: &TcContext, offset: usize) -> Result<*const T, ()> {
    // Calculate the position of a pointer in the packet's data based on the given offset.
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    // Check if accessing this pointer would go out of bounds.
    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn start_ebpfw(ctx: TcContext) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    match ethhdr.ether_type {
        EtherType::Ipv4 => {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            let addr = if is_ingress() {
                u32::from_be(ipv4hdr.src_addr)
            } else {
                u32::from_be(ipv4hdr.dst_addr)
            };

            // Check if the IP should be blocked using the block_ip function.
            if block_ip(addr) {
                info!(&ctx, "Blocking IP: {:i}", addr);
                return Ok(TC_ACT_SHOT); // Drop the packet
            }
        }
        _ => {}
    };

    // Allow the packet if no block conditions are met
    Ok(TC_ACT_PIPE) 
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
