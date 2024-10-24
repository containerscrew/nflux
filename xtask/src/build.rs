use std::{ffi::OsString, process::Command};

use anyhow::{bail, Context as _, Result};
use clap::Parser;
use xtask::AYA_BUILD_EBPF;

#[derive(Debug, Parser)]
pub struct Options {
    /// Build the release target.
    #[clap(long)]
    release: bool,
    /// The command used to wrap your application.
    #[clap(short, long, default_value = "sudo -E")]
    runner: String,
    /// Additional arguments to pass to the build command.
    #[clap(global = true, last = true)]
    build_args: Vec<OsString>,
}

/// Build the project.
pub fn build(opts: Options) -> Result<()> {
    let Options {
        release,
        runner,
        build_args,
    } = opts;

    let mut cmd = Command::new("cargo");
    cmd.env(AYA_BUILD_EBPF, "true");
    cmd.args(["build", "--package", "nflux", "--config"]);
    if release {
        cmd.arg(format!("target.\"cfg(all())\".runner=\"{}\"", runner));
        cmd.arg("--release");
    } else {
        cmd.arg(format!("target.\"cfg(all())\".runner=\"{}\"", runner));
    }
    if !build_args.is_empty() {
        cmd.arg("--").args(build_args);
    }
    let status = cmd
        .status()
        .with_context(|| format!("failed to build {cmd:?}"))?;
    if status.code() != Some(0) {
        bail!("{cmd:?} failed: {status:?}")
    }
    Ok(())
}
