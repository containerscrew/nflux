use aya_ebpf::{
    macros::map,
    maps::Array,
};
use aya_ebpf::maps::{LruHashMap, RingBuf};
use nflux_common::TcConfig;

#[map]
pub static TC_CONFIG: Array<TcConfig> = Array::with_max_entries(1, 0);

#[map]
pub static TC_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<u32, u32> = LruHashMap::with_max_entries(4096, 0);
