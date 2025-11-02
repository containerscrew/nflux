use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_type")]
    pub log_type: String,
    #[serde(default = "default_with_timer")]
    pub with_timer: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_type() -> String {
    "text".to_string()
}
fn default_with_timer() -> bool {
    false
}

impl LoggingConfig {
    pub fn validate(&self) -> Result<()> {
        let allowed_levels = ["trace", "debug", "info", "warn", "error"];
        if !allowed_levels.contains(&self.log_level.as_str()) {
            anyhow::bail!("Invalid log_level: '{}'", self.log_level);
        }
        let allowed_types = ["text", "json"];
        if !allowed_types.contains(&self.log_type.as_str()) {
            anyhow::bail!("Invalid log_type: '{}'", self.log_type);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AgentConfig {
    pub interface: String,
    pub mode: String,
    pub listen_port: Option<u16>,
    // pub exclude_ports: Option<Vec<u16>>,
    pub enable_udp: bool,
    pub enable_icmp: bool,
    pub enable_tcp: bool,
    pub enable_arp: bool,
    pub log_interval: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NfluxConfig {
    pub logging: LoggingConfig,
    pub agent: AgentConfig,
}

impl NfluxConfig {
    pub fn load(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Error reading the config file in '{}'", path))?;

        let config: NfluxConfig =
            toml::from_str(&contents).context("Error parsing TOML content")?;

        config.logging.validate()?;

        Ok(config)
    }
}
