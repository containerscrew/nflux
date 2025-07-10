use aya_ebpf::{macros::map, maps::RingBuf};

#[map]
pub static DROPPED_PACKETS_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);
