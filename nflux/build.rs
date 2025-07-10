use anyhow::{anyhow, Context as _};
use aya_build::cargo_metadata;

fn main() -> anyhow::Result<()> {
    let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .context("MetadataCommand::exec")?;

    let ebpf_tc = packages
        .iter()
        .find(|pkg| pkg.name == "ebpf-tc")
        .ok_or_else(|| anyhow!("nflux-ebpf-tc package not found"))?
        .clone();

    aya_build::build_ebpf([ebpf_tc])?;

    let ebpf_dp = packages
        .iter()
        .find(|pkg| pkg.name == "ebpf-dpkt")
        .ok_or_else(|| anyhow!("nflux-ebpf-dpkt package not found"))?
        .clone();

    aya_build::build_ebpf([ebpf_dp])?;

    let ebpf_cgroups = packages
        .iter()
        .find(|pkg| pkg.name == "ebpf-cgroups")
        .ok_or_else(|| anyhow!("ebpf-cgroups package not found"))?
        .clone();

    aya_build::build_ebpf([ebpf_cgroups])?;

    Ok(())
}
