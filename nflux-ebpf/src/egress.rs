use aya_ebpf::bindings::TC_ACT_PIPE;
use aya_ebpf::macros::map;
use aya_ebpf::maps::{LruHashMap, PerfEventArray};
use aya_ebpf::programs::TcContext;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::Ipv4Hdr;
use nflux_common::EgressEvent;


pub fn try_tc_egress(ctx: TcContext) -> Result<i32, ()> {
    let ethhdr: EthHdr = ctx.load(0).map_err(|_| ())?;
    match ethhdr.ether_type {
        EtherType::Ipv4 => unsafe {
            let ipv4hdr: Ipv4Hdr = ctx.load(EthHdr::LEN).map_err(|_| ())?;
            let destination = u32::from_be(ipv4hdr.dst_addr);

            // Check if this destination is already active
            if ACTIVE_CONNECTIONS.get(&destination).is_none() {
                // Log only new connections
                let event = EgressEvent { dst_ip: destination };
                EGRESS_EVENT.output(&ctx, &event, 0);

                // Mark connection as active
                if ACTIVE_CONNECTIONS.insert(&destination, &1, 0).is_err() {
                    return Err(());
                }
            }
        }
        _ => return Ok(TC_ACT_PIPE),
    }

    Ok(TC_ACT_PIPE)
}
