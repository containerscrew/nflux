use std::net::{IpAddr, Ipv4Addr};
use dns_lookup::lookup_addr;
use libc::getuid;
use tokio::signal;
use tracing::{info, warn};

// Check if the current user ID is 0 (root)
pub fn is_root_user() -> bool {
    unsafe { getuid() == 0 }
}

pub async fn wait_for_shutdown() -> anyhow::Result<()> {
    let ctrl_c = signal::ctrl_c();
    info!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    warn!("Exiting...");
    Ok(())
}

pub fn lookup_address(ip: u32) -> String {
    // Convert the u32 IP address to Ipv4Addr
    let ip = Ipv4Addr::from(ip);

    // Convert to IpAddr for compatibility with lookup_addr
    let ip = IpAddr::V4(ip);

    // Perform the reverse DNS lookup
    lookup_addr(&ip).unwrap_or_else(|_| "Unknown host".to_string())
}
