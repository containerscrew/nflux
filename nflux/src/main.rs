use std::{
    fs::File,
    process::{self, exit},
};

use anyhow::Context;
use aya::{
    include_bytes_aligned,
    maps::RingBuf,
    programs::{CgroupAttachMode, CgroupSkb, CgroupSkbAttachType},
    Ebpf,
};
use clap::Parser;
use libc::getuid;
use logger::LoggerConfig;
use nflux_common::Configmap;
use tracing::{error, info, warn};
use utils::{is_true, set_mem_limit};

use crate::{
    cli::NfluxCliArgs,
    containers::{ContainerRuntime, ContainerdRuntime, PodmanRuntime},
    logger::init_logger,
    network_event::{process_ring_buffer, DisplayNetworkEvent},
    programs::{start_dropped_packets, start_traffic_control},
    utils::{is_root_user, wait_for_shutdown},
};

mod cli;
mod containers;
mod events;
mod logger;
mod network_event;
mod programs;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = NfluxCliArgs::parse();

    init_logger(LoggerConfig {
        level: cli.log_level,
        format: cli.log_format.clone(),
        with_timer: cli.with_timer,
    });

    let uid = unsafe { getuid() };
    if let Err(e) = is_root_user(uid) {
        error!("{}", e);
        exit(1);
    }

    // Set memory limit for eBPF maps
    set_mem_limit();

    info!("Starting nflux with pid {}", process::id());

    // Load eBPF program
    let mut bpf_tc = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/tc")))?;
    let mut bpf_dp = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/dpkt")))?;
    let mut bpf_cgroups = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/cgroups")))?;

    // if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
    //     // This can happen if you remove all log statements from your eBPF program.
    //     warn!("failed to initialize eBPF logger: {e}");
    // }

    // Match possible subcommands
    match cli.command {
        Some(cli::Commands::Dpkt {}) => {
            info!("Sniffing dropped packets");
            start_dropped_packets(&mut bpf_dp, cli.log_format).await?;
        }
        Some(cli::Commands::Tc {
            interface,
            disable_egress,
            disable_ingress,
            listen_port,
            exclude_port,
            disable_udp,
            disable_icmp,
            disable_tcp,
            log_interval,
            disable_full_log,
        }) => {
            info!("Sniffing traffic on interface: {}", interface);
            let configmap = Configmap {
                disable_udp: is_true(disable_udp), // 0 = no, 1 = yes
                disable_icmp: is_true(disable_icmp),
                disable_tcp: is_true(disable_tcp),
                log_interval: log_interval as u64 * 1_000_000_000,
                disable_full_log: is_true(disable_full_log),
                listen_port: listen_port.unwrap_or(0), // Default to 0 if not provided
            };

            // If enable_egress and enable_ingress are both false, the app is doing nothing, exit
            if disable_egress && disable_ingress {
                error!("Can't disable both egress and ingress traffic, nothing to display.");
                exit(1)
            }

            // Also, if all protocols are disabled, exit
            if disable_icmp && disable_tcp && disable_udp {
                error!("You disabled all the protocols (tcp/udp/icmp), nothing to display.");
                exit(1)
            }

            // Start nflux tc
            start_traffic_control(
                &mut bpf_tc,
                &interface,
                disable_egress,
                disable_ingress,
                configmap,
                cli.log_format,
                exclude_port,
            )
            .await?;
        }
        Some(cli::Commands::Cgroups {
            cgroup_path: _,
            podman_socket_path,
            containerd_socket_path,
        }) => {
            info!("Sniffing container traffic using cgroup skb");

            start_cgroups_traffic(&mut bpf_cgroups, podman_socket_path, containerd_socket_path)
                .await?;
        }
        None => {
            // Unreachable: CLI shows help if no args are provided.
        }
    }

    Ok(())
}

async fn start_cgroups_traffic(
    ebpf: &mut Ebpf,
    podman_socket_path: String,
    containerd_socket_path: String,
) -> anyhow::Result<()> {
    // TODO: containerd support
    // First of all, list containers
    let containerd = ContainerdRuntime::new(&containerd_socket_path).await;
    let containerd_containers = containerd.list_containers().await?;

    for container in containerd_containers {
        println!("{}", container.name);
        println!("{}", container.cgroup_path);
    }

    let podman = PodmanRuntime::new(&podman_socket_path);
    let podman_containers = podman.list_containers().await?;

    for container in podman_containers {
        info!("Attaching eBPF program to container: {}", container.name);

        // Attach the eBPF program to the cgroup path
        let cgroup_path = container.cgroup_path;
        let cgroup_file = File::open(&cgroup_path)
            .with_context(|| format!("Failed to open cgroup file: {}", &cgroup_path))?;

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
