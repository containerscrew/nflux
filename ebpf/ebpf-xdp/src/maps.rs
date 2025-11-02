use aya_ebpf::{macros::map, maps::LruHashMap};
use nflux_common::dto::ActiveConnectionKey;

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<ActiveConnectionKey, u64> =
    LruHashMap::with_max_entries(4096, 0);
