use anyhow::Context;
use aya::{
    include_bytes_aligned,
    maps::{Array, RingBuf},
    programs::{tc, SchedClassifier, TcAttachType},
    Ebpf,
};
use nflux_common::Configmap;
use tracing::{debug, error, info};

use super::tc_event::process_event;
use crate::utils::wait_for_shutdown;

pub async fn start_netrace(
    interface: &str,
    enable_egress: bool,
    enable_ingress: bool,
    configmap: Configmap,
) -> anyhow::Result<()> {
    // Load eBPF program
    let mut ebpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    try_traffic_control(
        &mut ebpf,
        interface,
        enable_ingress,
        enable_egress,
        configmap,
    )?;

    let tc_event_ring_map = ebpf
        .take_map("TC_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer TC_EVENT map"))?;

    let ring_buf = RingBuf::try_from(tc_event_ring_map)?;

    tokio::spawn(async move { process_event(ring_buf).await });

    let _ = wait_for_shutdown().await;

    Ok(())
}

fn try_traffic_control(
    ebpf: &mut Ebpf,
    interface: &str,
    enable_ingress: bool,
    enable_egress: bool,
    configmap: Configmap,
) -> Result<(), anyhow::Error> {
    if enable_egress {
        attach_tc_program(ebpf, "tc_egress", interface, TcAttachType::Egress)?;
    }

    if enable_ingress {
        attach_tc_program(ebpf, "tc_ingress", interface, TcAttachType::Ingress)?;
    }

    // Populate config
    populate_configmap(ebpf, configmap)?;

    Ok(())
}

fn attach_tc_program(
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

fn populate_configmap(bpf: &mut Ebpf, config: Configmap) -> anyhow::Result<()> {
    let mut tc_config = Array::<_, Configmap>::try_from(
        bpf.map_mut("TC_CONFIG")
            .context("Failed to find TC_CONFIG map")?,
    )?;

    tc_config
        .set(0, config, 0)
        .context("Failed to set TC_CONFIG")?;

    debug!("eBPF map TC_CONFIG successfully loaded with struct Configmap");

    Ok(())
}
