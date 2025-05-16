use std::{
    ffi::CString,
    net::{IpAddr, Ipv4Addr},
};

use default_net::interface::get_default_interface_name;
use dns_lookup::lookup_addr;
use libc::{c_char, c_int, getservbyport, ntohs, servent, setrlimit};
use nflux_common::utils::is_private_ip;
use sysinfo::{Pid, System};
use tokio::signal;
use tracing::{info, warn};

/// is_root_user checks if the current user who runs the program is root.
/// Avoid running nflux as uid != 0 (root). Ebpf requires privileges
pub fn check_is_root_user(uid: u32) -> Result<(), String> {
    if uid != 0 {
        return Err(
            "This program must be run as root. Try: $ sudo nflux -i iface-name".to_string(),
        );
    }
    Ok(())
}

pub fn set_default_iface() -> String {
    match get_default_interface_name() {
        Some(iface) => {
            return iface
        },
        None => {
            return "No default interface found. Are you connected? Try: $ nflux netrace -i iface-name".to_string()
        }
    }
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

/// is_true converts a boolean value to a u8. In C terms, 1 is true and 0 is false.
pub fn is_true(value: bool) -> u8 {
    if value {
        return 1;
    }
    0
}

/// convert_protocol converts the protocol number to a string.
pub fn convert_protocol(protocol: u8) -> &'static str {
    match protocol {
        1 => "icmp",
        6 => "tcp",
        17 => "udp",
        _ => "unknown",
    }
}

pub fn _get_service_name(port: u16, proto: &'static str) -> String {
    let c_proto = CString::new(proto).unwrap_or_else(|_| CString::new("").unwrap());
    let c_port = ntohs(port);

    unsafe {
        let serv: *mut servent = getservbyport(c_port as c_int, c_proto.as_ptr() as *const c_char);
        if serv.is_null() {
            return "nodata".to_string();
        }

        let name = std::ffi::CStr::from_ptr((*serv).s_name)
            .to_string_lossy()
            .into_owned();
        name
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
        None => "nodata".to_string(),
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
            "This program must be run as root. Try: $ sudo nflux -i iface-name"
        );
    }

    #[test]
    fn test_is_true() {
        assert_eq!(is_true(true), 1);
        assert_eq!(is_true(false), 0);
    }
}
