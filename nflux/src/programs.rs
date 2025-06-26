use anyhow::Context;
use aya::{
    maps::{Array, RingBuf},
    programs::{tc, SchedClassifier, TcAttachType, TracePoint},
    Ebpf,
};
use nflux_common::Configmap;
use tokio::sync::watch;
use tracing::{debug, error};

use super::events::{process_dp_events, process_tc_events};
use crate::utils::wait_for_shutdown;

pub async fn start_dropped_packets(
    ebpf: &mut Ebpf,
    log_format: String,
) -> anyhow::Result<()> {
    let program: &mut TracePoint = ebpf.program_mut("dropped_packets").unwrap().try_into()?;
    program.load()?;
    program.attach("skb", "kfree_skb")?;

    let dropped_packets_ring_map = ebpf
        .take_map("DROPPED_PACKETS_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer DROPPED_PACKETS_EVENT map"))?;

    let ring_buf = RingBuf::try_from(dropped_packets_ring_map)?;

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let handle = tokio::spawn(async move {
        if let Err(e) = process_dp_events(ring_buf, shutdown_rx, log_format).await {
            error!("process_event failed: {:?}", e);
        }
    });

    wait_for_shutdown().await?;

    let _ = shutdown_tx.send(true);

    handle.await?;

    Ok(())
}

pub async fn start_traffic_control(
    ebpf: &mut Ebpf,
    interface: &str,
    disable_egress: bool,
    disable_ingress: bool,
    configmap: Configmap,
    log_format: String,
    exclude_ports: Option<Vec<u16>>,
) -> anyhow::Result<()> {
    try_traffic_control(ebpf, interface, disable_ingress, disable_egress, configmap)?;

    let tc_event_ring_map = ebpf
        .take_map("TC_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer TC_EVENT map"))?;

    let ring_buf = RingBuf::try_from(tc_event_ring_map)?;

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let handle = tokio::spawn(async move {
        if let Err(e) = process_tc_events(ring_buf, log_format, exclude_ports, shutdown_rx).await {
            error!("process_event failed: {:?}", e);
        }
    });

    wait_for_shutdown().await?;

    let _ = shutdown_tx.send(true);

    handle.await?;

    Ok(())
}

fn try_traffic_control(
    ebpf: &mut Ebpf,
    interface: &str,
    disable_ingress: bool,
    disable_egress: bool,
    configmap: Configmap,
) -> Result<(), anyhow::Error> {
    if !disable_egress {
        attach_tc_program(ebpf, "tc_egress", interface, TcAttachType::Egress)?;
    }

    if !disable_ingress {
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

    Ok(())
}

fn populate_configmap(
    bpf: &mut Ebpf,
    config: Configmap,
) -> anyhow::Result<()> {
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
