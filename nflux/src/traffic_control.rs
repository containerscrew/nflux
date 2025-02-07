use std::net::Ipv4Addr;
use std::ptr;
use std::sync::Arc;
use aya::{programs::{tc, SchedClassifier, TcAttachType}, Ebpf};
use aya::maps::MapData;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use bytes::BytesMut;
use tracing::{debug, error, info, warn};
use nflux_common::{convert_protocol, TcEvent};
use crate::metrics::Metrics;

pub fn start_traffic_control(bpf: &mut Ebpf, interfaces: Vec<String>) -> Result<(), anyhow::Error> {
    if interfaces.is_empty() {
        warn!("No interfaces provided to attach the TC program");
        return Ok(());
    }

    for (prog_name, attach_type) in [
        ("tc_egress", TcAttachType::Egress),
        ("tc_ingress", TcAttachType::Ingress),
    ] {
        attach_tc_program(bpf, prog_name, interfaces.as_slice(), attach_type)?;
    }
    Ok(())
}

// pub fn start_traffic_control(bpf: &mut Ebpf, config: Monitoring) -> Result<(), anyhow::Error> {
//     match config.enabled {
//         IsEnabled::True => {
//             if !config.physical_interfaces.is_empty() {
//                 info!(
//                     "Attaching TC egress program to physical interfaces: {:?}",
//                     config.physical_interfaces
//                 );
//                 attach_tc_program(
//                     bpf,
//                     "tc_egress_physical",
//                     &config.physical_interfaces,
//                     TcAttachType::Egress,
//                 )?;
//                 attach_tc_program(
//                     bpf,
//                     "tc_ingress_physical",
//                     &config.physical_interfaces,
//                     TcAttachType::Ingress,
//                 )?;
//             }

//             // Virtual interface is not working fine ATM
//             if !config.virtual_interfaces.is_empty() {
//                 info!(
//                     "Attaching TC egress program to virtual interfaces: {:?}",
//                     config.virtual_interfaces
//                 );
//                 attach_tc_program(
//                     bpf,
//                     "tc_egress_virtual",
//                     &config.virtual_interfaces,
//                     TcAttachType::Egress,
//                 )?;
//                 attach_tc_program(
//                     bpf,
//                     "tc_ingress_virtual",
//                     &config.virtual_interfaces,
//                     TcAttachType::Ingress,
//                 )?;
//             }
//             populate_egress_config(bpf, config)?;
//             info!("TC egress started successfully!")
//         }
//         IsEnabled::False => {
//             info!("Egress not enabled");
//         }
//     }
//     Ok(())
// }

// pub fn populate_egress_config(bpf: &mut Ebpf, config: Monitoring) -> anyhow::Result<()> {
//     let mut egress_config = Array::<_, EgressConfig>::try_from(
//         bpf.map_mut("EGRESS_CONFIG")
//             .context("Failed to find EGRESS_CONFIG map")?,
//     )?;

//     let config = EgressConfig {
//         log_only_new_connections: match config.logging.log_only_new_connections {
//             IsEnabled::True => 1,
//             IsEnabled::False => 0,
//         },
//         log_refresh_new_connections_every: config.logging.log_refresh_new_connections_every,
//         log_udp_connections: match config.logging.log_udp_connections {
//             IsEnabled::True => 1,
//             IsEnabled::False => 0,
//         },
//         log_tcp_connections: match config.logging.log_tcp_connections {
//             IsEnabled::True => 1,
//             IsEnabled::False => 0,
//         },
//         log_icmp_connections: match config.logging.log_icmp_connections {
//             IsEnabled::True => 1,
//             IsEnabled::False => 0,
//         },
//     };

//     egress_config
//         .set(0, config, 0)
//         .context("Failed to set ICMP_MAP")?;

//     Ok(())
// }

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
            debug!(
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

    info!("{} program attached to interfaces: {:?}", program_name, interfaces);

    Ok(())
}

pub async fn process_egress_events(
    mut buf: AsyncPerfEventArrayBuffer<MapData>,
    cpu_id: u32,
    metrics: Arc<Metrics>,
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
                    // Log the connection
                    info!(
                        "{} protocol={}, src_ip={}, dst_ip={}, src_port={}, dst_port={}",
                        if event.direction == 0 {"ingress"} else { "egress"},
                        convert_protocol(event.protocol),
                        Ipv4Addr::from(event.src_ip),
                        Ipv4Addr::from(event.dst_ip),
                        event.src_port,
                        event.dst_port,
                    );
                    // if event.direction == 0 {
                    //     metrics.track_ingress_event(
                    //         convert_protocol(event.protocol),
                    //         Ipv4Addr::from(event.src_ip).to_string().as_str(),
                    //         Ipv4Addr::from(event.dst_ip).to_string().as_str(),
                    //         event.src_port.to_string().as_str(),
                    //         event.dst_port.to_string().as_str(),
                    //     );
                    // } else {
                    //     metrics.track_egress_event(
                    //         convert_protocol(event.protocol),
                    //         Ipv4Addr::from(event.src_ip).to_string().as_str(),
                    //         Ipv4Addr::from(event.dst_ip).to_string().as_str(),
                    //         event.src_port.to_string().as_str(),
                    //         event.dst_port.to_string().as_str(),
                    //     );
                    // }
                }
                Err(e) => error!("Failed to parse egress event on CPU {}: {}", cpu_id, e),
            }
        }
    }
}

pub fn parse_egress_event(buf: &BytesMut) -> anyhow::Result<TcEvent> {
    if buf.len() >= std::mem::size_of::<TcEvent>() {
        let ptr = buf.as_ptr() as *const TcEvent;
        let event = unsafe { ptr::read_unaligned(ptr) };
        Ok(event)
    } else {
        Err(anyhow::anyhow!("Buffer size is too small for EgressEvent"))
    }
}
