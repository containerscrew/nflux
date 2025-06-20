use aya_ebpf::{
    bindings::socket, helpers::gen::bpf_get_current_pid_tgid, programs::TracePointContext,
};
use aya_log_ebpf::info;
use nflux_common::skb_reason::{reason_description, reason_to_str};

const REASON_OFFSET: usize = 36;
const PROTO_OFFSET: usize = 32;

const RX_SK_OFFSET: usize = 24;

// https://github.com/torvalds/linux/blob/master/include/net/dropreason-core.h
// tracepoint format: sudo cat /sys/kernel/debug/tracing/events/skb/kfree_skb/format
pub fn try_dropped_packets(ctx: TracePointContext) -> Result<u32, u32> {
    let reason_code = unsafe { ctx.read_at::<u32>(REASON_OFFSET).map_err(|_| 1u32)? };
    let proto = unsafe { ctx.read_at::<u16>(PROTO_OFFSET).map_err(|_| 1u32)? };
    let pid_tgid = unsafe { bpf_get_current_pid_tgid() };
    let pid = (pid_tgid >> 32) as u32;

    // Read the pointer to the socket structure
    let _sk_ptr = unsafe {
        ctx.read_at::<*const socket>(RX_SK_OFFSET)
            .map_err(|_| 1u32)?
    };

    let reason_str = reason_to_str(reason_code);
    let reason_desc = reason_description(reason_code);

    info!(
        &ctx,
        "Dropped packet! Proto: {} Reason Code: {} Reason: {} PID: {} Human friendly: {}",
        proto,
        reason_code,
        reason_str,
        pid,
        reason_desc,
    );
    Ok(0)
}
