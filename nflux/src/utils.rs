use std::net::{IpAddr, Ipv4Addr};

use default_net::interface::get_default_interface_name;
use dns_lookup::lookup_addr;
use libc::setrlimit;
use nflux_common::utils::is_private_ip;
use sysinfo::{Pid, System};
use tokio::signal;
use tracing::{info, warn};

/// is_root_user checks if the current user who runs the program is root.
pub fn check_is_root_user(uid: u32) -> Result<(), String> {
    if uid != 0 {
        return Err("This program must be run as root. Try: $ sudo nflux -i iface-name".to_string());
    }
    Ok(())
}

pub fn set_default_iface() -> Vec<String> {
    let mut default_ifaces: Vec<String> = Vec::new();
    let default_iface = get_default_interface_name().unwrap();
    default_ifaces.push(default_iface);
    default_ifaces
}

/// set_mem_limit bumps the memlock rlimit to infinity.
pub fn set_mem_limit() {
    // Bump the memlock rlimit
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
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

pub fn _get_process_name(pid: u32) -> String {
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
        None => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_is_root_user_with_root_uid() {
        let result = check_is_root_user(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_is_root_user_with_non_root_uid() {
        let result = check_is_root_user(1000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "This program must be run as root. Try: $ sudo nflux --help"
        );
    }
}
