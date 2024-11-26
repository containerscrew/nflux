mod cli;
mod config;
mod core;
mod logger;
mod utils;

use crate::cli::Args;
use crate::config::Config;
use crate::core::set_mem_limit;
use crate::utils::{is_root_user, wait_for_shutdown};
use anyhow::Context;
use aya::maps::perf::{AsyncPerfEventArrayBuffer, PerfBufferError};
use aya::maps::{Array, AsyncPerfEventArray, MapData};
use aya::programs::{Xdp, XdpFlags};
use aya::util::online_cpus;
use aya::{include_bytes_aligned, Ebpf};
use bytes::BytesMut;
use clap::Parser;
use logger::setup_logger;
use nflux_common::{
    convert_protocol, AppConfig, ConnectionEvent, MAX_ALLOWED_IPV4, MAX_ALLOWED_PORTS,
};
use std::net::Ipv4Addr;
use std::{env, ptr};
use tokio::task;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    // Parse command-line arguments
    let args = Args::parse();

    // Load configuration file
    let config = Config::load_config(args.config_file.as_str());

    // Enable logging
    setup_logger(&config.log.log_level, &config.log.log_type);

    // Check if user is root.
    if !is_root_user() {
        error!("This program must be run as root.");
        std::process::exit(1);
    }

    // Mem limit
    set_mem_limit();

    // Load eBPF program
    let mut bpf = Ebpf::load(include_bytes_aligned!(concat!(env!("OUT_DIR"), "/nflux")))?;

    // If you want to print logs from eBPF program, uncomment the following lines
    // if let Err(e) = aya_log::EbpfLogger::init(&mut bpf) {
    //     warn!("failed to initialize eBPF logger: {}", e);
    // }

    let app_config = AppConfig {
        allow_icmp: if config.firewall.allow_icmp { 1 } else { 0 },
        allowed_ipv4: convert_ipv4_vec_to_array(&config.firewall.allowed_ipv4, MAX_ALLOWED_IPV4),
        allowed_ports: convert_ports_vec_to_array(
            &config.firewall.allowed_ports,
            MAX_ALLOWED_PORTS,
        ),
    };

    // Populate EBPF map with app config
    load_app_config(&mut bpf, &app_config)?;

    // Attach XDP program
    // TODO: check if the interface you want to attach is valid (physical)
    // XDP program can only be attached to physical interfaces
    let program: &mut Xdp = bpf.program_mut("nflux").unwrap().try_into()?;
    program.load()?;
    program.attach(config.nflux.interface_name.as_str(), XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // Some basic info
    println!("\x1b[32mstarting nflux, a network traffic firewall and monitoring!\x1b[0m");
    info!(
        "Successfully attached XDP program to iface: {}",
        config.nflux.interface_name
    );
    info!("Checking incoming packets...");

    let mut events = AsyncPerfEventArray::try_from(bpf.take_map("CONNECTION_EVENTS").unwrap())?;
    let cpus = online_cpus().map_err(|(_, error)| error)?;

    for cpu_id in cpus {
        let buf = events.open(cpu_id, None)?;

        task::spawn(process_events(buf, cpu_id));
    }

    // Wait for shutdown signal
    wait_for_shutdown().await?;

    Ok(())
}

async fn process_events(
    mut buf: AsyncPerfEventArrayBuffer<MapData>,
    cpu_id: u32,
) -> Result<(), PerfBufferError> {
    let mut buffers = vec![BytesMut::with_capacity(1024); 10];

    loop {
        // Wait for events
        let events = buf.read_events(&mut buffers).await?;

        // Process each event in the buffer
        for i in 0..events.read {
            let buf = &buffers[i];
            match parse_connection_event(buf) {
                Ok(event) => {
                    info!(
                        "CPU={} program=xdp protocol={} port={} ip={}",
                        cpu_id,
                        convert_protocol(event.protocol),
                        event.dst_port,
                        Ipv4Addr::from(event.src_addr)
                    );
                }
                Err(e) => error!("Failed to parse event on CPU {}: {}", cpu_id, e),
            }
        }
    }
}

// Helper function to convert Vec<String> to [u32; N]
fn convert_ipv4_vec_to_array(vec: &Vec<String>, max_len: usize) -> [u32; MAX_ALLOWED_IPV4] {
    let mut array = [0; MAX_ALLOWED_IPV4];
    for (i, ip_str) in vec.iter().take(max_len).enumerate() {
        if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
            array[i] = u32::from(ip);
        }
    }
    array
}

fn load_app_config(bpf: &mut Ebpf, app_config: &AppConfig) -> anyhow::Result<()> {
    let mut app_config_map: Array<_, AppConfig> =
        Array::try_from(bpf.map_mut("APP_CONFIG").unwrap())?;
    app_config_map.set(0, app_config, 0)?;
    Ok(())
}

fn convert_ports_vec_to_array(vec: &Vec<u32>, max_len: usize) -> [u32; MAX_ALLOWED_PORTS] {
    let mut array = [0; MAX_ALLOWED_PORTS];
    for (i, &port) in vec.iter().take(max_len).enumerate() {
        array[i] = port;
    }
    array
}

fn parse_connection_event(buf: &BytesMut) -> anyhow::Result<ConnectionEvent> {
    if buf.len() >= std::mem::size_of::<ConnectionEvent>() {
        let ptr = buf.as_ptr() as *const ConnectionEvent;
        // Safety: we've confirmed the buffer is large enough
        let event = unsafe { ptr::read_unaligned(ptr) };
        Ok(event)
    } else {
        Err(anyhow::anyhow!(
            "Buffer size is too small for ConnectionEvent"
        ))
    }
}
