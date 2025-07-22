use aya_ebpf::{
    macros::map,
    maps::{Array, LruHashMap, RingBuf},
};
use nflux_common::dto::Configmap;

use crate::dto::ActiveConnectionKey;

#[map]
pub static TC_CONFIG: Array<Configmap> = Array::with_max_entries(1, 0);

#[map]
pub static NETWORK_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<ActiveConnectionKey, u64> =
    LruHashMap::with_max_entries(4096, 0);
