use aya_ebpf::{
    macros::map,
    maps::{Array, LruHashMap, RingBuf},
};
use nflux_common::Configmap;

// Define a struct for the key: (pid, destination IP)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ActiveConnectionKey {
    pub src_port: u32,
    pub dst_ip: u32,
}

#[map]
pub static TC_CONFIG: Array<Configmap> = Array::with_max_entries(1, 0);

#[map]
pub static TC_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static ACTIVE_CONNECTIONS: LruHashMap<ActiveConnectionKey, u8> =
    LruHashMap::with_max_entries(4096, 0);
