use std::{
    fmt::{self, Display},
    fs::File,
    process::{self, exit},
};

use anyhow::Context;
use aya::{
    include_bytes_aligned,
    maps::{MapData, RingBuf},
    programs::{CgroupAttachMode, CgroupSkb, CgroupSkbAttachType},
    Ebpf,
};
use clap::Parser;
use libc::getuid;
use logger::LoggerConfig;
use nflux_common::{Configmap, NetworkEvent};
use tracing::{error, info, warn};
use utils::{is_true, set_mem_limit};

use crate::{
    cli::NfluxCliArgs,
    containers::{ContainerRuntime, PodmanRuntime},
    logger::init_logger,
    programs::{start_dropped_packets, start_traffic_control},
    utils::{convert_direction, convert_protocol, is_root_user, to_ipaddr, wait_for_shutdown},
};

mod cli;
mod containers;
mod events;
mod logger;
mod programs;
mod utils;

// Supertrait to convert NetworkEvent to a Displayable format
struct DisplayNetworkEvent(pub NetworkEvent);

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
        }) => {
            info!("Sniffing container traffic using cgroup skb");
            // TODO: autodiscover cgroup path for containerd
            // let _channel = connect("/run/containerd/containerd.sock").await.unwrap();

            start_cgroups_traffic(&mut bpf_cgroups, podman_socket_path).await?;
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
) -> anyhow::Result<()> {
    // First of all, list containers
    let podman = PodmanRuntime::new(&podman_socket_path);

    let podman_containers = podman.list_containers(false).await?;

    let program: &mut CgroupSkb = ebpf.program_mut("cgroups_traffic").unwrap().try_into()?;
    program.load()?;

    for contaiener in podman_containers {
        info!("Attachingf eBPF program to container: {}", contaiener.name);

        // Attach the eBPF program to the cgroup path
        let cgroup_path = contaiener.cgroup_path;
        let cgroup_file = File::open(&cgroup_path)
            .with_context(|| format!("Failed to open cgroup file: {}", &cgroup_path))?;

        program.attach(
            &cgroup_file,
            CgroupSkbAttachType::Egress,
            CgroupAttachMode::default(),
        )?;
    }

    let network_event = ebpf
        .take_map("CGROUP_NETWORK_EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer CGROUP_NETWORK_EVENT map"))?;

    let ring_buf = RingBuf::try_from(network_event)?;

    tokio::select! {
        res = process_ring_buffer::<DisplayNetworkEvent>(ring_buf) => {
            if let Err(e) = res {
                error!("process_dp_events failed: {:?}", e);
            }
        },
        _ = wait_for_shutdown() => {
            warn!("You press Ctrl-C, shutting down nflux...");
        }
    }

    Ok(())
}

impl fmt::Display for DisplayNetworkEvent {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let event = &self.0;
        write!(
            f,
            "[{}][{}][{}] {}:{} -> {}:{} pkt_len={} ttl={}",
            convert_direction(event.direction),
            convert_protocol(event.protocol),
            event.ip_family.as_str(),
            to_ipaddr(event.src_ip, event.ip_family.to_owned()),
            event.src_port,
            to_ipaddr(event.dst_ip, event.ip_family.to_owned()),
            event.dst_port,
            event.total_len,
            event.ttl,
        )?;

        if let Some(flags) = event.tcp_flags {
            write!(f, ", tcp_flags: {:?}", flags)?;
        }

        write!(f, " }}")
    }
}

async fn process_ring_buffer<T>(mut ring_buf: RingBuf<MapData>) -> Result<(), anyhow::Error>
where
    T: Display,
{
    loop {
        while let Some(event) = ring_buf.next() {
            let data = event.as_ref();

            if data.len() == std::mem::size_of::<T>() {
                let event: &T = unsafe { &*(data.as_ptr() as *const T) };
                info!("{}", event);
            } else {
                warn!(
                    "Event size mismatch: expected {}, got {}",
                    std::mem::size_of::<T>(),
                    data.len()
                );
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
