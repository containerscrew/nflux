use std::{
    ffi::CString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use default_net::interface::get_default_interface_name;
use dns_lookup::lookup_addr;
use libc::{c_char, c_int, getservbyport, getuid, ntohs, servent, setrlimit};
use nflux_common::{dto::TcpFlags, utils::is_ipv4_private_address};
use sysinfo::{Pid, System};
use tokio::signal;
use tracing::{debug, warn};

/// check_is_root checks if the current user who runs the program is root
pub fn check_is_root() -> Result<(), anyhow::Error> {
    let uid = unsafe { getuid() };
    if uid != 0 {
        return Err(anyhow::anyhow!("nflux must be run as root".to_string()));
    }
    Ok(())
}

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

pub fn _to_ipaddr(
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

/// set_default_iface returns the default interface name.
pub fn _set_default_iface() -> String {
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

pub fn _lookup_ipv4_address(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ipv4) => {
            if is_ipv4_private_address(ipv4) {
                "Private IP".to_string()
            } else {
                lookup_addr(&IpAddr::V4(ipv4)).unwrap_or_else(|_| "unknown".to_string())
            }
        }
        IpAddr::V6(_) => "IPv6 not supported".to_string(),
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
