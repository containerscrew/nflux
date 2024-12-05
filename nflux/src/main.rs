mod config;
mod core;
mod logger;
mod utils;
mod ebpf_mapping;

use anyhow::Context;
use aya::maps::lpm_trie::Key;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use aya::maps::{AsyncPerfEventArray, LpmTrie, MapData};
use aya::programs::{Xdp, XdpFlags};
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use bytes::BytesMut;
use config::{Action, Nflux, Protocol, IpRules};
use core::set_mem_limit;
use logger::setup_logger;
use nflux_common::{convert_protocol, ConnectionEvent, IpRule, LpmKeyIpv4};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::ptr;
use tokio::task;
use tracing::{error, info};
use utils::{is_root_user, wait_for_shutdown};
use crate::ebpf_mapping::populate_icmp_rule;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration file
    let config = Nflux::load_config().context("Failed to load nflux configuration")?;

    // Enable logging
    setup_logger(&config.logging.log_level, &config.logging.log_type);

    // Ensure the program is run as root
    if !is_root_user() {
        error!("This program must be run as root.");
        std::process::exit(1);
    }

    // Set memory limit
    set_mem_limit();

    // Load eBPF program
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    // Populate eBPF maps with configuration data
    populate_ipv4_rules(&mut bpf, &config.ip_rules)?;
    populate_icmp_rule(&mut bpf, config.nflux.icmp_ping)?;
    // populate_ipv6_rules(&mut bpf, &config.ip_rules)?;

    // Attach XDP program
    let program: &mut Xdp = bpf.program_mut("nflux").unwrap().try_into()?;
    program.load()?;
    program
        .attach(&config.nflux.interface_names[0], XdpFlags::default())
        .context(
            "Failed to attach XDP program. Ensure the interface is physical and not virtual.",
        )?;

    // Log startup info
    info!("nflux started successfully!");
    info!(
        "XDP program attached to interface: {:?}",
        config.nflux.interface_names[0]
    );

    // Start processing events from the eBPF program
    let mut events = AsyncPerfEventArray::try_from(
        bpf.take_map("CONNECTION_EVENTS")
            .context("Failed to find CONNECTION_EVENTS map")?,
    )?;
    let cpus = online_cpus().map_err(|(_, error)| error)?;

    for cpu_id in cpus {
        let buf = events.open(cpu_id, None)?;
        task::spawn(process_events(buf, cpu_id));
    }

    // Wait for shutdown signal
    wait_for_shutdown().await?;
    Ok(())
}

async fn process_events(
    mut buf: AsyncPerfEventArrayBuffer<MapData>,
    cpu_id: u32,
) -> Result<(), PerfBufferError> {
    let mut buffers = vec![BytesMut::with_capacity(1024); 10];

    loop {
        // Wait for events
        let events = buf.read_events(&mut buffers).await?;

        // Process each event in the buffer
        for i in 0..events.read {
            let buf = &buffers[i];
            match parse_connection_event(buf) {
                Ok(event) => {
                    info!(
                        "CPU={} program=xdp protocol={} port={} ip={}",
                        cpu_id,
                        convert_protocol(event.protocol),
                        event.dst_port,
                        Ipv4Addr::from(event.src_addr),
                    );
                }
                Err(e) => error!("Failed to parse event on CPU {}: {}", cpu_id, e),
            }
        }
    }
}

fn parse_connection_event(buf: &BytesMut) -> anyhow::Result<ConnectionEvent> {
    if buf.len() >= std::mem::size_of::<ConnectionEvent>() {
        let ptr = buf.as_ptr() as *const ConnectionEvent;
        let event = unsafe { ptr::read_unaligned(ptr) };
        Ok(event)
    } else {
        Err(anyhow::anyhow!(
            "Buffer size is too small for ConnectionEvent"
        ))
    }
}

fn populate_ipv4_rules(bpf: &mut Ebpf, ip_rules: &HashMap<String, IpRules>) -> anyhow::Result<()> {
    let mut ipv4_map: LpmTrie<&mut MapData, LpmKeyIpv4, IpRule> = LpmTrie::try_from(
        bpf.map_mut("IPV4_RULES")
            .context("Failed to find IPV4_RULES map")?,
    )?;

    // Sort rules by priority
    let mut sorted_rules: Vec<_> = ip_rules.iter().collect();
    sorted_rules.sort_by_key(|(_, rule)| rule.priority);

    for (cidr, rule) in sorted_rules {
        let (ip, prefix_len) = parse_cidr_v4(cidr)?;
        let ip_rule = prepare_ip_rule(rule)?;

        let key = Key::new(
            prefix_len,
            LpmKeyIpv4 {
                prefix_len,
                ip: ip.into(),
            },
        );

        ipv4_map
            .insert(&key, &ip_rule, 0)
            .context("Failed to insert IPv4 rule")?;
    }

    Ok(())
}

fn prepare_ip_rule(rule: &IpRules) -> anyhow::Result<IpRule> {
    let mut ports = [0u16; 16];
    for (i, &port) in rule.ports.iter().enumerate().take(16) {
        ports[i] = port as u16;
    }

    Ok(IpRule {
        action: match rule.action {
            Action::Allow => 1,
            Action::Deny => 0,
        },
        ports,
        protocol: match rule.protocol {
            Protocol::Tcp => 6,
            Protocol::Udp => 17,
        },
        priority: rule.priority,
    })
}

// fn populate_ipv6_rules(bpf: &mut Ebpf, ip_rules: &HashMap<String, Rules>) -> anyhow::Result<()> {
//     let mut ipv6_map: LpmTrie<&mut MapData, LpmKeyIpv6, IpRule> = LpmTrie::try_from(
//         bpf.map_mut("IPV6_RULES").context("Failed to find IPV4_RULES map")?,
//     )?;

//     for (cidr, rule) in ip_rules {
//         let (ip, prefix_len) = parse_cidr_v6(cidr)?;
//         let ip_rule = prepare_ip_rule(rule)?;

//         let key = Key::new(prefix_len, LpmKeyIpv6 { prefix_len, ip: ip.into() });
//         ipv6_map.insert(&key, &ip_rule, 0).context("Failed to insert IPv6 rule")?;
//     }

//     Ok(())
// }

fn parse_cidr_v4(cidr: &str) -> anyhow::Result<(Ipv4Addr, u32)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid CIDR format: {}", cidr));
    }
    let ip = parts[0].parse::<Ipv4Addr>()?;
    let prefix_len = parts[1].parse::<u32>()?;
    Ok((ip, prefix_len))
}

// fn parse_cidr_v6(cidr: &str) -> anyhow::Result<(Ipv6Addr, u32)> {
//     let parts: Vec<&str> = cidr.split('/').collect();
//     if parts.len() != 2 {
//         return Err(anyhow::anyhow!("Invalid CIDR format: {}", cidr));
//     }
//     let ip = parts[0].parse::<Ipv6Addr>()?;
//     let prefix_len = parts[1].parse::<u32>()?;
//     Ok((ip, prefix_len))
// }
