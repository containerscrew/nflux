#![no_std]
#![no_main]

use aya_ebpf::{

    helpers::{bpf_get_current_comm, bpf_get_current_pid_tgid, bpf_probe_read_buf}, macros::{map, uprobe, uretprobe}, maps::{HashMap, PerCpuArray, RingBuf}, programs::{ProbeContext, RetProbeContext}
};
use nflux_common::{Kind, TLSData};


#[map]
static mut STORAGE: PerCpuArray<TLSData> = PerCpuArray::with_max_entries(1, 0);

#[map]
pub static EVENT: RingBuf = RingBuf::with_byte_size(1024, 0);

#[map]
static mut BUFFERS: HashMap<u32, *const u8> = HashMap::with_max_entries(1024, 0);

#[uprobe]
pub fn ssl_read(ctx: ProbeContext) -> u32 {
    match try_ssl(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[uretprobe]
pub fn ssl_read_ret(ctx: RetProbeContext) -> u32 {
    match try_ssl_ret(ctx, Kind::Read) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[uprobe]
pub fn ssl_write(ctx: ProbeContext) -> u32 {
    match try_ssl(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[uretprobe]
pub fn ssl_write_ret(ctx: RetProbeContext) -> u32 {
    match try_ssl_ret(ctx, Kind::Write) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

// Support for libns
#[uprobe]
pub fn pr_read(ctx: ProbeContext) -> u32 {
    match try_ssl(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[uretprobe]
pub fn pr_read_ret(ctx: RetProbeContext) -> u32 {
    match try_ssl_ret(ctx, Kind::Read) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[uprobe]
pub fn pr_write(ctx: ProbeContext) -> u32 {
    match try_ssl(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[uretprobe]
pub fn pr_write_ret(ctx: RetProbeContext) -> u32 {
    match try_ssl_ret(ctx, Kind::Write) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

// `try_ssl` function is an eBPF probe for capturing SSL data.
fn try_ssl(ctx: ProbeContext) -> Result<u32, u32> {
    let tgid: u32 = (bpf_get_current_pid_tgid() >> 32) as u32;

    // Get the buffer pointer (second argument of the probed function) from the context.
    let buf_p: *const u8 = ctx.arg(1).ok_or(0_u32)?;

    // Insert the buffer pointer into the `BUFFERS` map for the current process/thread group.
    unsafe { BUFFERS.insert(&tgid, &buf_p, 0).ok(); }

    Ok(0)
}

// `try_ssl_ret` function is an eBPF probe for handling the return value of an SSL function.
fn try_ssl_ret(ctx: RetProbeContext, kind: Kind) -> Result<u32, u32> {
    let retval: i32 = ctx.ret().ok_or(0u32)?;

    if retval <= 0 {
        return Ok(0);
    }

    let tgid: u32 = (bpf_get_current_pid_tgid() >> 32) as u32;

    let buf_p = unsafe {
        let ptr = BUFFERS.get(&tgid).ok_or(0_u32)?;
        *ptr
    };

    if buf_p.is_null() {
        return Ok(0);
    }

    let event = unsafe {
        let ptr = STORAGE.get_ptr_mut(0).ok_or(0_u32)?;
        &mut *ptr
    };

    // Populate the `Data` structure with the required data.
    event.kind = kind;
    event.len = retval;
    event.comm = bpf_get_current_comm().map_err(|e| e as u32)?;

    //let buffer_limit = retval.min(MAX_BUF_SIZE as i32) as u32;

    unsafe {
        let ret = bpf_probe_read_buf(buf_p, event.buf.as_mut());

        match ret {
            Ok(_) => {
            }
            Err(ret) => {
                return Err(0);
            }
        }

        BUFFERS.remove(&tgid).ok();

        if let Some(mut data) = EVENT.reserve::<TLSData>(0) {
            *data.as_mut_ptr() = *event;
            data.submit(0);

        } else {
            return Err(0);
        }
    }

    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
