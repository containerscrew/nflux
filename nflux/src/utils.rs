use std::{
    ffi::CString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use default_net::interface::get_default_interface_name;
use dns_lookup::lookup_addr;
use libc::{c_char, c_int, getservbyport, ntohs, servent, setrlimit};
use nflux_common::{utils::is_private_ip, TcpFlags};
use sysinfo::{Pid, System};
use tokio::signal;
use tracing::{debug, warn};

pub fn convert_direction(direction: u8) -> &'static str {
    match direction {
        0 => "ingress",
        1 => "egress",
        _ => "unknown",
    }
}

/// convert_protocol converts the protocol number to a string.
pub fn convert_protocol(protocol: u8) -> &'static str {
    match protocol {
        1 => "icmp",
        6 => "tcp",
        17 => "udp",
        _ => {
            warn!("Unknown protocol: {}", protocol);
            "unknown"
        }
    }
}

pub fn to_ipaddr(
    ip: [u8; 16],
    ip_family: u8,
) -> IpAddr {
    match ip_family {
        2 => IpAddr::V4(Ipv4Addr::new(ip[12], ip[13], ip[14], ip[15])), // AF_INET
        10 => IpAddr::V6(Ipv6Addr::from(ip)),                           // AF_INET6
        _ => {
            warn!("Unknown ip_family: {}", ip_family);
            IpAddr::V4(Ipv4Addr::UNSPECIFIED)
        }
    }
}

/// is_root_user checks if the current user who runs the program is root.
/// Avoid running nflux as uid != 0 (root). eBPF requires privileges
pub fn is_root_user(uid: u32) -> Result<(), String> {
    if uid != 0 {
        return Err(
            "This program must be run as root. Try: $ sudo nflux subcommands [flags]".to_string(),
        );
    }
    Ok(())
}

/// set_default_iface returns the default interface name.
pub fn set_default_iface() -> String {
    match get_default_interface_name() {
        Some(iface) => iface,
        None => "No default interface found. Are you connected?".to_string(),
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

pub async fn wait_for_shutdown() -> Result<(), anyhow::Error> {
    let ctrl_c = signal::ctrl_c();
    debug!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    Ok(())
}

pub fn format_tcp_flags(flags: TcpFlags) -> String {
    let mut out = String::from("");
    let mut first = true;

    if flags.syn != 0 {
        out.push_str("SYN");
        first = false;
    }
    if flags.ack != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("ACK");
        first = false;
    }
    if flags.fin != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("FIN");
        first = false;
    }
    if flags.rst != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("RST");
    }
    if flags.psh != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("PSH");
    }
    if flags.urg != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("URG");
    }
    if flags.ece != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("ECE");
    }
    if flags.cwr != 0 {
        if !first {
            out.push_str(",");
        }
        out.push_str("CWR");
    }
    out
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

pub fn _get_service_name(
    port: u16,
    proto: &'static str,
) -> String {
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
        None => "notfound".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_is_root_user_with_root_uid() {
        let result = is_root_user(0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_is_root_user_with_non_root_uid() {
        let result = is_root_user(1000);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "This program must be run as root. Try: $ sudo nflux subcommands [flags]"
        );
    }

    #[test]
    fn test_is_true() {
        assert_eq!(is_true(true), 1);
        assert_eq!(is_true(false), 0);
    }

    #[test]
    fn test_convert_direction() {
        assert_eq!(convert_direction(0), "ingress");
        assert_eq!(convert_direction(1), "egress");
        assert_eq!(convert_direction(2), "unknown");
    }
}
