#![no_std]
#[derive(Debug)]
pub enum Action {
    Dropped,
    Accepted,
    Unknown,
}

impl From<i32> for Action {
    fn from(value: i32) -> Self {
        match value {
            2 => Action::Dropped,
            3 => Action::Accepted,
            _ => Action::Unknown,
        }
    }
}

impl Action {
    pub fn to_str(&self) -> &'static str {
        match self {
            Action::Dropped => "dropped",
            Action::Accepted => "accepted",
            Action::Unknown => "unknown",
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PacketLog {
    pub ipv4_address: u32,
    pub action: u32,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for PacketLog {}
pub const MAX_FIREWALL_RULES: u32 = 32;
pub const MAX_RULES_PORT: usize = 32;
pub const MAX_ALLOWED_PORTS: usize = 1024;
pub const MAX_ALLOWED_IPV4: usize = 1024;
