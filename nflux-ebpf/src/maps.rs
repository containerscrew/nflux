use aya_ebpf::{
    macros::map,
    maps::{Array, RingBuf},
};
use nflux_common::Configmap;

#[map]
pub static TC_CONFIG: Array<Configmap> = Array::with_max_entries(1, 0);

#[map]
pub static TC_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);
