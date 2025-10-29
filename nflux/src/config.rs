use std::fs;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_level: String,
    pub log_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub interface: String,
}

#[derive(Debug, Deserialize)]
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

        Ok(config)
    }
}
