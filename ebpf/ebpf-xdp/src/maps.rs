use aya_ebpf::{
    macros::map,
    maps::{LruHashMap, RingBuf},
};
use nflux_common::dto::ActiveConnectionKey;

#[map]
pub static XDP_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<ActiveConnectionKey, u64> =
    LruHashMap::with_max_entries(4096, 0);
