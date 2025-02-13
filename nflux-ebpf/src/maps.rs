use aya_ebpf::{
    macros::map,
    maps::{Array, LruHashMap, RingBuf},
};
use nflux_common::TcConfig;

use crate::logger::ConnectionKey;

#[map]
pub static TC_CONFIG: Array<TcConfig> = Array::with_max_entries(1, 0);

#[map]
pub static TC_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<ConnectionKey, u8> =
    LruHashMap::with_max_entries(4096, 0);
