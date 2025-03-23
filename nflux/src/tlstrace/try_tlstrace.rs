use aya::{include_bytes_aligned, maps::RingBuf, programs::UProbe, Ebpf};

use super::tls_event::process_tls_event;
use crate::utils::wait_for_shutdown;

fn attach_program(ebpf: &mut Ebpf, openssl_path: &str, pid: Option<i32>) -> anyhow::Result<()> {
    let program: &mut UProbe = ebpf.program_mut("ssl_write").unwrap().try_into()?;
    program.load()?;
    program.attach(Some("SSL_write"), 0, openssl_path, pid)?;

    let program: &mut UProbe = ebpf.program_mut("ssl_read").unwrap().try_into()?;
    program.load()?;
    program.attach(Some("SSL_read"), 0, openssl_path, pid)?;

    // Attach uretprobes
    let p_write_ret: &mut UProbe = ebpf.program_mut("ssl_write_ret").unwrap().try_into()?;
    p_write_ret.load()?;
    p_write_ret.attach(Some("SSL_write"), 0, openssl_path, pid)?;

    let p_write_ret: &mut UProbe = ebpf.program_mut("ssl_read_ret").unwrap().try_into()?;
    p_write_ret.load()?;
    p_write_ret.attach(Some("SSL_read"), 0, openssl_path, pid)?;

    Ok(())
}

pub async fn start_tlstrace(openssl_path: &str, pid: Option<i32>) -> anyhow::Result<()> {
    // Load eBPF program
    let mut ebpf = Ebpf::load(include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/tlstrace"
    )))?;

    attach_program(&mut ebpf, openssl_path, pid)?;

    try_tlstrace(&mut ebpf).await?;

    Ok(())
}

async fn try_tlstrace(ebpf: &mut Ebpf) -> anyhow::Result<()> {
    let event_ring_map = ebpf
        .take_map("EVENT")
        .ok_or_else(|| anyhow::anyhow!("Failed to find ring buffer EVENT map"))?;

    let ring_buf = RingBuf::try_from(event_ring_map)?;

    tokio::spawn(async move { process_tls_event(ring_buf).await });

    let _ = wait_for_shutdown().await?;
    Ok(())
}
