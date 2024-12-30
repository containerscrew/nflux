mod config;
mod core;
mod logger;
mod utils;
mod ebpf_mapping;

use anyhow::Context;
use aya::maps::lpm_trie::Key;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use aya::maps::{AsyncPerfEventArray, LpmTrie, MapData};
use aya::programs::{tc, SchedClassifier, TcAttachType, Xdp, XdpFlags};
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use aya_log::EbpfLogger;
use bytes::BytesMut;
use config::{Action, FirewallRules, IsEnabled, Nflux, Protocol};
use log::warn;
use core::set_mem_limit;
use logger::setup_logger;
use nflux_common::{convert_protocol, ConnectionEvent, EgressEvent, IpRule, LpmKeyIpv4, LpmKeyIpv6};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr;
use tokio::task;
use tracing::{error, info};
use utils::{is_root_user, lookup_address, wait_for_shutdown};
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

    // Necessary to debug something in the ebpf code
    // By the moment
    if let Err(e) = EbpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    // Attach XDP program
    match config.firewall.enabled {
        IsEnabled::True => {
            // Populate eBPF maps with configuration data
            populate_ip_rules(&mut bpf, &config.firewall_rules)?;
            populate_icmp_rule(&mut bpf, config.firewall.icmp_ping)?;

            // Load the XDP program
            let program: &mut Xdp = bpf.program_mut("nflux").unwrap().try_into()?;
            program.load()?;

            // Attach the XDP program to multiple interfaces
            for interface in &config.firewall.interfaces {
                if let Err(e) = program.attach(interface, XdpFlags::default()) {
                    error!(
                        "Failed to attach XDP program to interface {}: {}. Ensure it is a physical interface.",
                        interface, e
                    );
                } else {
                    info!("XDP program attached to interface: {}", interface);
                }
            }
        }
        IsEnabled::False => {
            // If not enabled, just log
            info!("Firewall not enabled")
        }
    }


    // Attach TC program
    // match config.egress.enabled {
    //     IsEnabled::True => {
    //         // Add clsact qdisc
    //         if let Err(e) = tc::qdisc_add_clsact(&config.egress.interface_name) {
    //             warn!(
    //                 "Failed to add clsact qdisc to interface {:?}: {:?}",
    //                 config.egress.interface_name, e
    //             );
    //         }

    //         // Get the tc_egress program
    //         let program = match bpf.program_mut("tc_egress") {
    //             Some(p) => p,
    //             None => {
    //                 error!("Failed to find the tc_egress program in BPF object");
    //                 return Err(anyhow::anyhow!("tc_egress program not found"));
    //             }
    //         };

    //         // Try converting the program into a SchedClassifier
    //         let program: &mut SchedClassifier = match program.try_into() {
    //             Ok(p) => p,
    //             Err(e) => {
    //                 error!("Failed to convert tc_egress program to SchedClassifier: {:?}", e);
    //                 return Err(e.into());
    //             }
    //         };

    //         // Load the program
    //         if let Err(e) = program.load() {
    //             error!("Failed to load tc_egress program: {:?}", e);
    //             return Err(e.into());
    //         }

    //         // Attach the program
    //         if let Err(e) = program.attach(&config.egress.interface_name, TcAttachType::Egress) {
    //             error!(
    //                 "Failed to attach tc_egress program to interface {:?}: {:?}",
    //                 config.egress.interface_name, e
    //             );
    //             return Err(e.into());
    //         }

    //         info!(
    //             "TC egress program successfully attached to interface: {:?}",
    //             config.egress.interface_name
    //         );
    //     }
    //     IsEnabled::False => {
    //         info!("Egress not enabled");
    //     }
    // }

    // Log startup info
    info!("nflux started successfully!");

    // Start processing events from the eBPF program
    let mut events = AsyncPerfEventArray::try_from(
        bpf.take_map("CONNECTION_EVENTS")
            .context("Failed to find CONNECTION_EVENTS map")?,
    )?;

    // let mut egress_events = AsyncPerfEventArray::try_from(
    //     bpf.take_map("EGRESS_EVENT")
    //         .context("Failed to find EGRESS_EVENT map")?,
    // )?;

    let cpus = online_cpus().map_err(|(_, error)| error)?;

    for cpu_id in cpus {
        // Spawn task for connection events
        {
            let buf = events.open(cpu_id, None)?;
            task::spawn(process_events(buf, cpu_id));
        }

        // // Spawn task for egress events
        // {
        //     let buf = egress_events.open(cpu_id, None)?;
        //     task::spawn(process_egress_events(buf, cpu_id));
        // }
    }

    // Wait for shutdown signal
    wait_for_shutdown().await?;
    Ok(())
}

async fn process_egress_events(
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
            match parse_egress_event(buf) {
                Ok(event) => {
                    info!(
                        "direction=outgoing ip={}, port={}, fqdn={}",
                        Ipv4Addr::from(event.dst_ip),
                        event.dst_port,
                        lookup_address(event.dst_ip),
                    );
                }
                Err(e) => error!("Failed to parse egress event on CPU {}: {}", cpu_id, e),
            }
        }
    }
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
                        "direction=incoming protocol={} port={} ip={} action={}",
                        convert_protocol(event.protocol),
                        event.dst_port,
                        Ipv4Addr::from(event.src_addr),
                        if event.action == 1 { "allow" } else { "deny" }
                    );
                }
                Err(e) => error!("Failed to parse event on CPU {}: {}", cpu_id, e),
            }
        }
    }
}

fn parse_egress_event(buf: &BytesMut) -> anyhow::Result<EgressEvent> {
    if buf.len() >= std::mem::size_of::<EgressEvent>() {
        let ptr = buf.as_ptr() as *const EgressEvent;
        let event = unsafe { ptr::read_unaligned(ptr) };
        Ok(event)
    } else {
        Err(anyhow::anyhow!(
            "Buffer size is too small for EgressEvent"
        ))
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

fn prepare_ip_rule(rule: &FirewallRules) -> anyhow::Result<IpRule> {
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

fn parse_cidr_v4(cidr: &str) -> anyhow::Result<(Ipv4Addr, u32)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid CIDR format: {}", cidr));
    }
    let ip = parts[0].parse::<Ipv4Addr>()?;
    let prefix_len = parts[1].parse::<u32>()?;
    Ok((ip, prefix_len))
}

fn parse_cidr_v6(cidr: &str) -> anyhow::Result<(Ipv6Addr, u32)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid CIDR format: {}", cidr));
    }
    let ip = parts[0].parse::<Ipv6Addr>()?;
    let prefix_len = parts[1].parse::<u32>()?;
    Ok((ip, prefix_len))
}

fn populate_ip_rules(bpf: &mut Ebpf, firewall_rules: &HashMap<String, FirewallRules>) -> anyhow::Result<()> {
    {
        // Populate IPv4 rules
        let mut ipv4_map: LpmTrie<&mut MapData, LpmKeyIpv4, IpRule> = LpmTrie::try_from(
            bpf.map_mut("IPV4_RULES")
                .context("Failed to find IPV4_RULES map")?,
        )?;

        // Sort rules by priority
        let mut sorted_rules: Vec<_> = firewall_rules.iter().collect();
        sorted_rules.sort_by_key(|(_, rule)| rule.priority);

        for (cidr, rule) in &sorted_rules {
            if let Ok((ip, prefix_len)) = parse_cidr_v4(cidr) {
                // Handle IPv4 rules
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
        }
    }

    {
        // Populate IPv6 rules
        let mut ipv6_map: LpmTrie<&mut MapData, LpmKeyIpv6, IpRule> = LpmTrie::try_from(
            bpf.map_mut("IPV6_RULES")
                .context("Failed to find IPV6_RULES map")?,
        )?;

        // Sort rules by priority
        let mut sorted_rules: Vec<_> = firewall_rules.iter().collect();
        sorted_rules.sort_by_key(|(_, rule)| rule.priority);

        for (cidr, rule) in &sorted_rules {
            if let Ok((ip, prefix_len)) = parse_cidr_v6(cidr) {
                // Handle IPv6 rules
                let ip_rule = prepare_ip_rule(rule)?;
                let key = Key::new(
                    prefix_len,
                    LpmKeyIpv6 {
                        prefix_len,
                        ip: ip.octets(),
                    },
                );
                ipv6_map
                    .insert(&key, &ip_rule, 0)
                    .context("Failed to insert IPv6 rule")?;
            }
        }
    }

    Ok(())
}
