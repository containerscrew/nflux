use core::net::Ipv4Addr;

use crate::dto::IpFamily;

// Checks if a given IPv4 address in u32 format is private.
// Returns `true` if the IP is private, otherwise `false`.
pub fn is_ipv4_private_address(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();

    match octets {
        [10, ..] => true,           // 10.0.0.0/8
        [172, 16..=31, ..] => true, // 172.16.0.0/12
        [192, 168, ..] => true,     // 192.168.0.0/16
        [127, ..] => true,          // 127.0.0.0/8
        [169, 254, ..] => true,     // 169.254.0.0/16
        _ => false,
    }
}

impl IpFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            IpFamily::Ipv4 => "IPv4",
            IpFamily::Ipv6 => "IPv6",
            IpFamily::Unknown => "Unknown",
        }
    }

    pub fn to_owned(&self) -> u8 {
        match self {
            IpFamily::Ipv4 => 2,    // AF_INET
            IpFamily::Ipv6 => 10,   // AF_INET6
            IpFamily::Unknown => 0, // Unknown
        }
    }

    pub fn from_u8(family: u8) -> Result<Self, u8> {
        match family {
            2 => Ok(IpFamily::Ipv4),  // AF_INET
            10 => Ok(IpFamily::Ipv6), // AF_INET6
            _ => Err(family),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_ips() {
        assert!(is_ipv4_private_address(Ipv4Addr::new(192, 168, 1, 10)));
        assert!(is_ipv4_private_address(Ipv4Addr::new(172, 16, 0, 1)));
        assert!(is_ipv4_private_address(Ipv4Addr::new(192, 168, 0, 1)));
        assert!(is_ipv4_private_address(Ipv4Addr::new(127, 0, 0, 1)));
        assert!(is_ipv4_private_address(Ipv4Addr::new(169, 254, 0, 1)));
    }

    #[test]
    fn test_public_ips() {
        assert!(!is_ipv4_private_address(Ipv4Addr::new(8, 8, 8, 8)));
        assert!(!is_ipv4_private_address(Ipv4Addr::new(1, 1, 1, 1)));
        assert!(!is_ipv4_private_address(Ipv4Addr::new(4, 4, 4, 4)));
    }
}
