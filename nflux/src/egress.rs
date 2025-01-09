use std::net::Ipv4Addr;
use std::ptr;
use aya::Ebpf;
use aya::maps::MapData;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use aya::programs::{tc, SchedClassifier, TcAttachType};
use bytes::BytesMut;
use tracing::{debug, error, info, warn};
use nflux_common::{convert_protocol, EgressEvent};
use crate::config::IsEnabled;
use crate::utils::{is_private_ip, lookup_address};

pub fn attach_tc_egress_program(bpf: &mut Ebpf, interface_names: &[String]) -> anyhow::Result<()>{
    // Retrieve the eBPF program
    let program = match bpf.program_mut("tc_egress") {
        Some(p) => p,
        None => {
            error!("Failed to find the tc_egress program in BPF object");
            return Err(anyhow::anyhow!("tc_egress program not found"));
        }
    };

    // Try converting the program into a SchedClassifier
    let program: &mut SchedClassifier = match program.try_into() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to convert tc_egress program to SchedClassifier: {:?}", e);
            return Err(e.into());
        }
    };

    // Load the eBPF program into the kernel
    if let Err(e) = program.load() {
        error!("Failed to load tc_egress program: {:?}", e);
        return Err(e.into());
    }

    // Iterate over interfaces and attach the program
    for interface in interface_names {
        // Add clsact qdisc to the interface
        if let Err(e) = tc::qdisc_add_clsact(interface) {
            warn!(
                "Failed to add clsact qdisc to interface {}: {:?}",
                interface, e
            );
        }

        // Attach the eBPF program to the egress path of the interface
        if let Err(e) = program.attach(interface, TcAttachType::Egress) {
            error!(
                "Failed to attach tc_egress program to interface {}: {:?}",
                interface, e
            );
            return Err(anyhow::anyhow!(
                "Failed to attach tc_egress program to interface {}",
                interface
            ));
        }
    }

    Ok(())
}

pub async fn process_egress_events(
    mut buf: AsyncPerfEventArrayBuffer<MapData>,
    cpu_id: u32,
    log_private_connections: &IsEnabled,
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
                    match log_private_connections {
                        IsEnabled::True => {
                             info!(
                                "program=tc_egress protocol={}, ip={}, src_port={}, dst_port={}, fqdn={}",
                                convert_protocol(event.protocol),
                                Ipv4Addr::from(event.dst_ip),
                                event.src_port,
                                event.dst_port,
                                "Private IP",
                            );
                        }
                        IsEnabled::False => {
                            if ! is_private_ip(event.dst_ip) {
                             info!(
                                "program=tc_egress protocol={}, ip={}, src_port={}, dst_port={}, fqdn={}",
                                convert_protocol(event.protocol),
                                Ipv4Addr::from(event.dst_ip),
                                event.src_port,
                                event.dst_port,
                                lookup_address(event.dst_ip),
                            );
                        }
                        }
                    }
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
        Err(anyhow::anyhow!(
            "Buffer size is too small for EgressEvent"
        ))
    }
}
