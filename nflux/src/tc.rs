use std::net::Ipv4Addr;

use anyhow::Context;
use aya::{
    maps::{Array, MapData, RingBuf},
    programs::{tc, SchedClassifier, TcAttachType},
    Ebpf,
};
use nflux_common::{convert_protocol, TcConfig, TcEvent};
use tracing::{debug, error, info};

use crate::utils::get_service_name;

pub fn start_traffic_control(
    bpf: &mut Ebpf,
    interface: String,
    enable_ingress: bool,
    disable_egress: bool,
    configmap: TcConfig,
) -> Result<(), anyhow::Error> {
    if !disable_egress {
        attach_tc_program(
            bpf,
            "tc_egress",
            interface.as_str(),
            TcAttachType::Egress,
        )?;
    }

    if enable_ingress {
        attach_tc_program(
            bpf,
            "tc_ingress",
            interface.as_str(),
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
    interface: &str,
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

    info!(
        "{} program attached to interface {}",
        program_name, interface
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
                let mut service_name = String::new();

                // Get service by name
                match get_service_name(event.dst_port, convert_protocol(event.protocol)) {
                    Some(name) => {
                        service_name.push_str(name.as_str());
                    }
                    None => {
                        service_name.push_str("unknown");
                    }
                }

                let direction = if event.direction == 0 {
                    "ingress"
                } else {
                    "egress"
                };

                info!(
                    "direction={} type={}, pid={}, protocol={}, serv={}, total_len={}B, ttl={}, src_ip={}, dst_ip={}, src_port={}, dst_port={}, iface=nodata",
                    direction,
                    event.ip_type.as_str(),
                    event.pid,
                    convert_protocol(event.protocol),
                    service_name,
                    event.total_len,
                    event.ttl,
                    Ipv4Addr::from(event.src_ip),
                    Ipv4Addr::from(event.dst_ip),
                    event.src_port,
                    event.dst_port,
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
