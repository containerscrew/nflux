use std::net::Ipv4Addr;

use anyhow::Context;
use aya::{
    maps::{Array, MapData, RingBuf},
    programs::{tc, SchedClassifier, TcAttachType},
    Ebpf,
};
use nflux_common::{convert_protocol, TcConfig, TcEvent};
use tracing::{debug, error, info};

use crate::utils::get_process_name;

pub fn start_traffic_control(
    bpf: &mut Ebpf,
    interfaces: Vec<String>,
    enable_ingress: bool,
    disable_egress: bool,
    configmap: TcConfig,
) -> Result<(), anyhow::Error> {
    if !disable_egress {
        attach_tc_program(
            bpf,
            "tc_egress",
            interfaces.as_slice(),
            TcAttachType::Egress,
        )?;
    }

    if enable_ingress {
        attach_tc_program(
            bpf,
            "tc_ingress",
            interfaces.as_slice(),
            TcAttachType::Ingress,
        )?;
    }

    // Populate config
    populate_egress_config(bpf, configmap)?;
    Ok(())
}

pub fn populate_egress_config(bpf: &mut Ebpf, config: TcConfig) -> anyhow::Result<()> {
    let mut tc_config = Array::<_, TcConfig>::try_from(
        bpf.map_mut("TC_CONFIG")
            .context("Failed to find TC_CONFIG map")?,
    )?;

    tc_config
        .set(0, config, 0)
        .context("Failed to set TC_CONFIG")?;

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

    info!(
        "{} program attached to interfaces: {:?}",
        program_name, interfaces
    );

    Ok(())
}

pub async fn process_event(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error> {
    loop {
        while let Some(event) = ring_buf.next() {
            // Get the data from the event
            let data = event.as_ref();

            // Make sure the data is the correct size
            if data.len() == std::mem::size_of::<TcEvent>() {
                let event: &TcEvent = unsafe { &*(data.as_ptr() as *const TcEvent) };
                // Log the connection
                let command = get_process_name(event.pid);
                info!(
                    "direction={} protocol={}, src_ip={}, dst_ip={}, src_port={}, dst_port={}, pid={}, comm={}",
                    if event.direction == 0 {"ingress"} else { "egress"},
                    convert_protocol(event.protocol),
                    Ipv4Addr::from(event.src_ip),
                    Ipv4Addr::from(event.dst_ip),
                    event.src_port,
                    event.dst_port,
                    event.pid,
                    command,
                );

                // if ! is_private_ip(event.dst_ip) {
                //     get_geolocation(Ipv4Addr::from(event.dst_ip).to_string().as_str()).await;
                // }
            }
        }

        // Sleep for a while
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
