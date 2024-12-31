use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::ptr;

use anyhow::Context;
use aya::maps::lpm_trie::Key;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use aya::Ebpf;
use aya::maps::{Array, LpmTrie, MapData};
use bytes::BytesMut;
use nflux_common::{convert_protocol, ConnectionEvent, IpRule, LpmKeyIpv4, LpmKeyIpv6};
use tracing::{error, info};
use crate::config::{Action, FirewallRules, IsEnabled, Protocol};
use crate::utils::{parse_cidr_v4, parse_cidr_v6};

pub fn populate_icmp_rule(bpf: &mut Ebpf, icmp_ping: IsEnabled) -> anyhow::Result<()> {
    let mut settings_map = Array::<_, u32>::try_from(
        bpf.map_mut("ICMP_RULE").context("Failed to find GLOBAL_SETTINGS map")?,
    )?;

    let value = match icmp_ping {
        IsEnabled::True => 1,
        IsEnabled::False => 0,
    };

    settings_map
        .set(0, value, 0)
        .context("Failed to set ICMP_MAP")?;

    Ok(())
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

pub fn attach_xdp_program(bpf: &mut Ebpf, icmp_enabled: IsEnabled, rules: &HashMap<String, FirewallRules>, interfaces: &Vec<String>) -> anyhow::Result<()> {
    // Populate eBPF maps with configuration data
    populate_ip_rules(bpf, &rules)?;
    populate_icmp_rule(bpf, icmp_enabled)?;

    // Load the XDP program
    let program: &mut aya::programs::Xdp = bpf.program_mut("xdp_firewall").unwrap().try_into()?;
    program.load()?;

    // Attach the XDP program to multiple interfaces
    for interface in interfaces {
        if let Err(e) = program.attach(interface, aya::programs::XdpFlags::default()) {
            error!(
                "Failed to attach XDP program to interface {}: {}. Ensure it is a physical interface.",
                interface, e
            );
        } else {
            info!("XDP program attached to interface: {}", interface);
        }
    }

    Ok(())
}

pub fn populate_ip_rules(bpf: &mut Ebpf, firewall_rules: &HashMap<String, FirewallRules>) -> anyhow::Result<()> {
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


// Process events
pub async fn process_firewall_events(
    mut buf: AsyncPerfEventArrayBuffer<MapData>,
    cpu_id: u32,
) -> Result<(), PerfBufferError> {
    let mut buffers = vec![BytesMut::with_capacity(4096); 10];

    loop {
        // Wait for events
        let events = buf.read_events(&mut buffers).await?;

        // Process each event in the buffer
        for i in 0..events.read {
            let buf = &buffers[i];
            match parse_firewall_event(buf) {
                Ok(event) => {
                    info!(
                        "program=xdp_firewall protocol={} port={} ip={} action={}",
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

fn parse_firewall_event(buf: &BytesMut) -> anyhow::Result<ConnectionEvent> {
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
