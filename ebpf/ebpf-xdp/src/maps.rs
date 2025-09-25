use aya_ebpf::{macros::map, maps::RingBuf};

#[map]
pub static XDP_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);
