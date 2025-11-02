use aya_ebpf::{macros::map, maps::RingBuf};

// Maps
#[map]
pub static NETWORK_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);
