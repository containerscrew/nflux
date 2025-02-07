use aya_ebpf::{
    macros::map,
    maps::{Array, PerfEventArray},
};
use nflux_common::{TcConfig, TcEvent};

#[map]
pub static TC_CONFIG: Array<TcConfig> = Array::with_max_entries(1, 0);

#[map]
pub static TC_EVENT: PerfEventArray<TcEvent> = PerfEventArray::new(0);

// #[map]
// pub static TC_EVENT: RingBuf = RingBuf::with_byte_size(4096, 0);
