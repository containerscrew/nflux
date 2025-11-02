use aya_ebpf::{
    macros::map,
    maps::{Array, RingBuf},
};

use crate::dto::Configmap;

// Maps
#[map]
pub static NETWORK_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);

#[map]
pub static CONFIGMAP: Array<Configmap> = Array::with_max_entries(1, 0);
