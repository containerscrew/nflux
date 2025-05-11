use anyhow::{anyhow, Context as _};
use aya_build::cargo_metadata;

fn main() -> anyhow::Result<()> {
    let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .context("MetadataCommand::exec")?;

    let ebpf_package = packages
        .iter()
        .find(|pkg| pkg.name == "netrace-ebpf")
        .ok_or_else(|| anyhow!("netrace-ebpf package not found"))?
        .clone();

    aya_build::build_ebpf([ebpf_package])
}
