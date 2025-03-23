use aya::{include_bytes_aligned, Ebpf};

pub async fn start_tlstrace() -> anyhow::Result<()> {
    // Load eBPF program
    let mut ebpf = Ebpf::load(include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/tlstrace"
    )))?;

    Ok()
}
