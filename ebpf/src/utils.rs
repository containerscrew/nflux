use core::mem;

use aya_ebpf::programs::TcContext;

#[inline]
pub fn tc_ptr_at<T>(
    ctx: &TcContext,
    offset: usize,
) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}