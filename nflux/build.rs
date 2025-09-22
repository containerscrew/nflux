use anyhow::{Context as _, anyhow};
use aya_build::cargo_metadata;

fn main() -> anyhow::Result<()> {
    let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .context("MetadataCommand::exec")?;

    let ebpf_packages: Vec<_> = packages
        .into_iter()
        .filter(|pkg| pkg.name.starts_with("ebpf-"))
        .collect();

    if ebpf_packages.is_empty() {
        return Err(anyhow!(
            "No eBPF packages found (expected names starting with nflux-ebpf-)"
        ));
    }

    aya_build::build_ebpf(ebpf_packages)?;
    Ok(())
}
