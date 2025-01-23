use crate::config::{IsEnabled, Monitoring};
use anyhow::Context;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use aya::maps::{Array, MapData};
use aya::programs::{tc, SchedClassifier, TcAttachType};
use aya::Ebpf;
use bytes::BytesMut;
use nflux_common::{convert_protocol, EgressConfig, EgressEvent};
use std::net::Ipv4Addr;
use std::ptr;
use tracing::{error, info, warn};

pub fn populate_egress_config(bpf: &mut Ebpf, config: Monitoring) -> anyhow::Result<()> {
    let mut egress_config = Array::<_, EgressConfig>::try_from(
        bpf.map_mut("EGRESS_CONFIG")
            .context("Failed to find EGRESS_CONFIG map")?,
    )?;

    let config = EgressConfig {
        log_only_new_connections: match config.logging.log_only_new_connections {
            IsEnabled::True => 1,
            IsEnabled::False => 0,
        },
        log_refresh_new_connections_every: config.logging.log_refresh_new_connections_every,
        log_udp_connections: match config.logging.log_udp_connections {
            IsEnabled::True => 1,
            IsEnabled::False => 0,
        },
        log_tcp_connections: match config.logging.log_tcp_connections {
            IsEnabled::True => 1,
            IsEnabled::False => 0,
        },
        log_icmp_connections: match config.logging.log_icmp_connections {
            IsEnabled::True => 1,
            IsEnabled::False => 0,
        },
    };

    egress_config
        .set(0, config, 0)
        .context("Failed to set ICMP_MAP")?;

    Ok(())
}

pub fn attach_tc_program(
    bpf: &mut Ebpf,
    program_name: &str,
    interfaces: &[String],
    attach_type: TcAttachType,
) -> anyhow::Result<()> {
    // Retrieve the eBPF program
    let program = match bpf.program_mut(program_name) {
        Some(p) => p,
        None => {
            error!("Failed to find the {} program in BPF object", program_name);
            return Err(anyhow::anyhow!("{} program not found", program_name));
        }
    };

    // Try converting the program into a SchedClassifier
    let program: &mut SchedClassifier = match program.try_into() {
        Ok(p) => p,
        Err(e) => {
            error!(
                "Failed to convert {} program to SchedClassifier: {:?}",
                program_name, e
            );
            return Err(e.into());
        }
    };

    // Load the eBPF program into the kernel
    if let Err(e) = program.load() {
        error!("Failed to load {} program: {:?}", program_name, e);
        return Err(e.into());
    }

    // Iterate over interfaces and attach the program
    for interface in interfaces {
        // Add clsact qdisc to the interface
        if let Err(e) = tc::qdisc_add_clsact(interface) {
            warn!(
                "Failed to add clsact qdisc to interface {}: {:?}",
                interface, e
            );
        }

        // Attach the eBPF program to the egress path of the interface
        if let Err(e) = program.attach(interface, attach_type) {
            error!(
                "Failed to attach {} program to interface {}: {:?}",
                program_name, interface, e
            );
            return Err(anyhow::anyhow!(
                "Failed to attach {} program to interface {}",
                program_name,
                interface
            ));
        }
    }

    Ok(())
}

pub async fn process_egress_events(
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
                        "{} protocol={}, src_ip={}, dst_ip={}, src_port={}, dst_port={}",
                        if event.direction == 0 {"ingress"} else { "egress"},
                        convert_protocol(event.protocol),
                        Ipv4Addr::from(event.src_ip),
                        Ipv4Addr::from(event.dst_ip),
                        event.src_port,
                        event.dst_port,
                    );
                }
                Err(e) => error!("Failed to parse egress event on CPU {}: {}", cpu_id, e),
            }
        }
    }
}

pub fn parse_egress_event(buf: &BytesMut) -> anyhow::Result<EgressEvent> {
    if buf.len() >= std::mem::size_of::<EgressEvent>() {
        let ptr = buf.as_ptr() as *const EgressEvent;
        let event = unsafe { ptr::read_unaligned(ptr) };
        Ok(event)
    } else {
        Err(anyhow::anyhow!("Buffer size is too small for EgressEvent"))
    }
}
