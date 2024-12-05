use anyhow::Context;
use aya::Ebpf;
use aya::maps::{Array};
use crate::config::IsEnabled;

pub fn populate_icmp_rule(bpf: &mut Ebpf, icmp_ping: IsEnabled) -> anyhow::Result<()> {
    let mut settings_map = Array::<_, u32>::try_from(
        bpf.map_mut("ICMP_RULE").context("Failed to find GLOBAL_SETTINGS map")?,
    )?;

    let value = match icmp_ping {
        IsEnabled::True => 1,
        IsEnabled::False => 0,
    };

    settings_map
        .set(0, value, 0)
        .context("Failed to set ICMP_MAP")?;

    Ok(())
}
