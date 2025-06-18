use aya_ebpf::{
    macros::map,
    maps::{
        Array,
        LruHashMap,
        RingBuf,
    },
};
use nflux_common::Configmap;

// Define a struct for the key of the active connections map
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ActiveConnectionKey {
    pub protocol: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub src_ip: [u8; 16],
    pub dst_ip: [u8; 16],
}

#[map]
pub static TC_CONFIG: Array<Configmap> = Array::with_max_entries(1, 0);

#[map]
pub static TC_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<ActiveConnectionKey, u64> =
    LruHashMap::with_max_entries(4096, 0);
