// Checks if a given IPv4 address in u32 format is private.
// Returns `true` if the IP is private, otherwise `false`.
pub fn is_private_ip(ip: u32) -> bool {
    // Convert u32 to the octet form for easier comparison
    let octets = [
        (ip >> 24) as u8, // First octet
        (ip >> 16) as u8, // Second octet
        (ip >> 8) as u8,  // Third octet
        ip as u8,         // Fourth octet
    ];

    // Check for private IP ranges
    match octets {
        [10, ..] => true,                             // 10.0.0.0/8
        [172, 16..=31, ..] => true,                   // 172.16.0.0/12
        [192, 168, ..] => true,                       // 192.168.0.0/16
        [127, ..] => true,                            // 127.0.0.0/8 (loopback)
        [169, 254, ..] => true,                       // 169.254.0.0/16 (link-local)
        _ => false,                                   // All others are public
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_ips() {
        assert!(is_private_ip(0x0A000001)); // 10.0.0.1
        assert!(is_private_ip(0xAC100001)); // 172.16.0.1
        assert!(is_private_ip(0xC0A80001)); // 192.168.0.1
        assert!(is_private_ip(0x7F000001)); // 127.0.0.1
        assert!(is_private_ip(0xA9FE0001)); // 169.254.0.1
    }

    #[test]
    fn test_public_ips() {
        assert!(!is_private_ip(0x08080808)); // 8.8.8.8
        assert!(!is_private_ip(0xC0000201)); // 192.0.2.1
        assert!(!is_private_ip(0x64400001)); // 100.64.0.1
    }
}
