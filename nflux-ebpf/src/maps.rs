use aya_ebpf::{macros::map, maps::{Array, LpmTrie, LruHashMap, PerfEventArray}};
use nflux_common::{ConnectionEvent, IpRule, LpmKeyIpv4, LpmKeyIpv6};

#[map]
pub static IPV4_RULES: LpmTrie<LpmKeyIpv4, IpRule> = LpmTrie::with_max_entries(1024, 0);

#[map]
pub static IPV6_RULES: LpmTrie<LpmKeyIpv6, IpRule> = LpmTrie::with_max_entries(1024, 0);

#[map]
pub static ICMP_RULE: Array<u32> = Array::with_max_entries(1, 0);

#[map]
pub static CONNECTION_EVENTS: PerfEventArray<ConnectionEvent> = PerfEventArray::new(0);

#[map]
pub static CONNECTION_TRACKER: LruHashMap<u64, u64> = LruHashMap::with_max_entries(1024, 0);
