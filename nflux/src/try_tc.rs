use anyhow::Context;
use aya::{
    maps::Array,
    programs::{tc, SchedClassifier, TcAttachType},
    Ebpf,
};
use nflux_common::Configmap;
use tracing::{debug, error, info};

pub fn try_traffic_control(
    bpf: &mut Ebpf,
    interface: String,
    enable_ingress: bool,
    enable_egress: bool,
    configmap: Configmap,
) -> Result<(), anyhow::Error> {
    if enable_egress {
        attach_tc_program(bpf, "tc_egress", interface.as_str(), TcAttachType::Egress)?;
    }

    if enable_ingress {
        attach_tc_program(bpf, "tc_ingress", interface.as_str(), TcAttachType::Ingress)?;
    }

    // Populate config
    populate_configmap(bpf, configmap)?;

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

pub fn populate_configmap(bpf: &mut Ebpf, config: Configmap) -> anyhow::Result<()> {
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
