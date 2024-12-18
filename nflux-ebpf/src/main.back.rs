#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]


use aya_ebpf::bindings::xdp_action::{XDP_DROP, XDP_PASS};
use aya_ebpf::helpers::bpf_ktime_get_ns;
use aya_ebpf::maps::lpm_trie::Key;
use aya_ebpf::maps::{Array, LpmTrie, LruHashMap};
use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::PerfEventArray,
    programs::XdpContext,
};
use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
use nflux_common::{ConnectionEvent, IpRule, LpmKeyIpv4, LpmKeyIpv6};

#[map]
static IPV4_RULES: LpmTrie<LpmKeyIpv4, IpRule> = LpmTrie::with_max_entries(1024, 0);

#[map]
static IPV6_RULES: LpmTrie<LpmKeyIpv6, IpRule> = LpmTrie::with_max_entries(1024, 0);

#[map]
static ICMP_RULE: Array<u32> = Array::with_max_entries(1, 0);

#[map]
static CONNECTION_EVENTS: PerfEventArray<ConnectionEvent> = PerfEventArray::new(0);

#[map]
static CONNECTION_TRACKER: LruHashMap<u64, u64> = LruHashMap::with_max_entries(1024, 0);

#[xdp]
pub fn nflux(ctx: XdpContext) -> u32 {
    match start_nflux(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn log_new_connection(ctx: &XdpContext, src_addr: u32, dst_port: u16, protocol: u8, action: u8) {
    let event = ConnectionEvent {
        src_addr,
        dst_port,
        protocol,
        action,
    };

    CONNECTION_EVENTS.output(ctx, &event, 0);
}

fn start_nflux(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };

    match unsafe { (*ethhdr).ether_type } {
        EtherType::Ipv4 => process_ipv4(&ctx),
        _ => Ok(XDP_DROP),
    }
}

fn process_ipv4(ctx: &XdpContext) -> Result<u32, ()> {
    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(ctx, EthHdr::LEN)? };
    let source_ip = u32::from_be(unsafe { (*ipv4hdr).src_addr });
    let dest_ip = u32::from_be(unsafe { (*ipv4hdr).dst_addr });
    let proto = unsafe { (*ipv4hdr).proto };

    // Direct lookup for exact match
    if let Some(rule) = IPV4_RULES.get(&Key::new(32, LpmKeyIpv4 { prefix_len: 32, ip: source_ip })) {
        return handle_protocol(ctx, proto, source_ip, dest_ip, rule);
    }

    // Prefix-based matching
    let mut mask = 0xFFFFFFFF;
    for prefix_len in (1..32).rev() {
        mask <<= 1;
        let key = Key::new(prefix_len, LpmKeyIpv4 { prefix_len, ip: source_ip & mask });
        if let Some(rule) = IPV4_RULES.get(&key) {
            return handle_protocol(ctx, proto, source_ip, dest_ip, rule);
        }
    }

    Ok(XDP_DROP)
}

fn handle_protocol(
    ctx: &XdpContext,
    proto: IpProto,
    source_ip: u32,
    dest_ip: u32,
    rule: &IpRule,
) -> Result<u32, ()> {
    match proto {
        IpProto::Tcp => handle_tcp(ctx, source_ip, dest_ip, rule),
        IpProto::Udp => handle_udp(ctx, source_ip, rule),
        IpProto::Icmp => handle_icmp(ctx, source_ip),
        _ => Ok(XDP_DROP),
    }
}

fn handle_tcp(
    ctx: &XdpContext,
    source_ip: u32,
    dest_ip: u32,
    rule: &IpRule,
) -> Result<u32, ()> {
    let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
    let dst_port = u16::from_be(unsafe { (*tcphdr).dest });
    let syn = unsafe { (*tcphdr).syn() };
    let ack = unsafe { (*tcphdr).ack() };

    let connection_key = ((source_ip as u64) << 32) | (dest_ip as u64);

    // Allow packets for established connections
    if unsafe { CONNECTION_TRACKER.get(&connection_key).is_some() } {
        return Ok(XDP_PASS);
    }

    // Handle new connections (SYN packets)
    if syn == 1 && ack == 0 {
        if rule.ports.contains(&dst_port) && rule.action == 1 {
            let timestamp = unsafe { bpf_ktime_get_ns() };
            let _ = CONNECTION_TRACKER.insert(&connection_key, &timestamp, 0);
            log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 1);
            return Ok(XDP_PASS);
        } else {
            log_new_connection(ctx, source_ip, dst_port, IpProto::Tcp as u8, 0);
            return Ok(XDP_DROP);
        }
    }

    Ok(XDP_DROP)
}

fn handle_udp(ctx: &XdpContext, source_ip: u32, rule: &IpRule) -> Result<u32, ()> {
    let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
    let dst_port = u16::from_be(unsafe { (*udphdr).dest });

    if rule.ports.contains(&dst_port) && rule.action == 1 {
        log_new_connection(ctx, source_ip, dst_port, IpProto::Udp as u8, 1);
        return Ok(XDP_PASS);
    }

    log_new_connection(ctx, source_ip, dst_port, IpProto::Udp as u8, 0);
    Ok(XDP_DROP)
}

fn handle_icmp(ctx: &XdpContext, source_ip: u32) -> Result<u32, ()> {
    if let Some(&icmp_ping) = ICMP_RULE.get(0) {
        if icmp_ping == 1 {
            log_new_connection(ctx, source_ip, 0, IpProto::Icmp as u8, 1);
            return Ok(XDP_PASS);
        }
    }

    log_new_connection(ctx, source_ip, 0, IpProto::Icmp as u8, 0);
    Ok(XDP_DROP)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
