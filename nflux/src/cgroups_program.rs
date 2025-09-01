use std::fs::File;

use aya::{
    maps::RingBuf,
    programs::{CgroupAttachMode, CgroupSkb, CgroupSkbAttachType},
    Ebpf,
};
use tracing::{error, info, warn};

use crate::{
    containers::{ContainerRuntime, PodmanRuntime},
    network_event::{process_ring_buffer, DisplayNetworkEvent},
    utils::wait_for_shutdown,
};

pub async fn _start_cgroups_traffic(
    ebpf: &mut Ebpf,
    podman_socket_path: String,
    _containerd_socket_path: String,
) -> anyhow::Result<()> {
    // TODO: containerd support
    // First of all, list containers
    let podman = PodmanRuntime::new(&podman_socket_path);
    let podman_containers = podman.list_containers().await?;
    // let containerd = ContainerdRuntime::new(&containerd_socket_path).await;
    // let _containerd_containers = containerd.list_containers().await?;

    for container in podman_containers {
        info!("Attaching eBPF program to container: {}", container.name);

        // Attach the eBPF program to the cgroup path
        let cgroup_path = container.cgroup_path;
        let cgroup_file = File::open(&cgroup_path)?;

        attach_skb_program(
            ebpf,
            "cgroups_traffic_egress",
            CgroupSkbAttachType::Egress,
            &cgroup_file,
        )
        .await?;

        attach_skb_program(
            ebpf,
            "cgroups_traffic_ingress",
            CgroupSkbAttachType::Ingress,
            &cgroup_file,
        )
        .await?;
    }

    let network_event = ebpf
        .take_map("CGROUP_NETWORK_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer CGROUP_NETWORK_EVENT map"))?;

    let ring_buf = RingBuf::try_from(network_event)?;

    tokio::select! {
        res = process_ring_buffer::<DisplayNetworkEvent>(ring_buf) => {
            if let Err(e) = res {
                error!("Processing cgroup packets {:?}", e);
            }
        },
        _ = wait_for_shutdown() => {
            warn!("You press Ctrl-C, shutting down nflux...");
        }
    }

    Ok(())
}

async fn attach_skb_program(
    ebpf: &mut Ebpf,
    program_name: &str,
    attach_type: CgroupSkbAttachType,
    cgroup_file: &File,
) -> anyhow::Result<()> {
    let program: &mut CgroupSkb = ebpf.program_mut(program_name).unwrap().try_into()?;
    program.load()?;

    program.attach(cgroup_file, attach_type, CgroupAttachMode::default())?;

    Ok(())
}
