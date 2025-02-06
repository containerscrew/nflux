use dns_lookup::lookup_addr;
use libc::getuid;
use nflux_common::utils::is_private_ip;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};
use sysinfo::{Pid, System};
use tokio::signal;
use tracing::{info, warn};


// Check if the current user ID is 0 (root)
pub fn is_root_user() -> bool {
    unsafe { getuid() == 0 }
}

pub fn set_mem_limit() {
    // Bump the memlock rlimit
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        warn!("remove limit on locked memory failed, ret is: {}", ret);
    }
}

pub async fn wait_for_shutdown() -> anyhow::Result<()> {
    let ctrl_c = signal::ctrl_c();
    info!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    warn!("You press Ctrl-C, shutting down nflux...");
    Ok(())
}

pub fn parse_cidr_v4(cidr: &str) -> anyhow::Result<(Ipv4Addr, u32)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid CIDR format: {}", cidr));
    }
    let ip = parts[0].parse::<Ipv4Addr>()?;
    let prefix_len = parts[1].parse::<u32>()?;
    Ok((ip, prefix_len))
}

pub fn parse_cidr_v6(cidr: &str) -> anyhow::Result<(Ipv6Addr, u32)> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid CIDR format: {}", cidr));
    }
    let ip = parts[0].parse::<Ipv6Addr>()?;
    let prefix_len = parts[1].parse::<u32>()?;
    Ok((ip, prefix_len))
}

pub fn _lookup_address(ip: u32) -> String {
    match is_private_ip(ip) {
        true => "Private IP".to_string(),
        false => {
            // Convert the u32 IP address to Ipv4Addr
            let ip = Ipv4Addr::from(ip);

            // Convert to IpAddr for compatibility with lookup_addr
            let ip = IpAddr::V4(ip);

            // Perform the reverse DNS lookup
            lookup_addr(&ip).unwrap_or_else(|_| "Unknown host".to_string())
        }
    }
}

pub fn _get_process_name(pid: u64) -> String {
    let mut s = System::new_all();

    // Is this causing overhead?
    s.refresh_all();

    match s.process(Pid::from(pid as usize)) {
        Some(process) => {
            // Get the process name. Remove "" when have spaces or other special characters
            format!("{:?}", process.name())
                .to_string()
                .trim_matches('"')
                .to_string()
        }
        None => String::new(),
    }
}
