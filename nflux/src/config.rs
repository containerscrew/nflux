use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;

/// Enum for `action`
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Deny,
    Allow,
}

/// Enum for `protocol`
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
}

// General firewall configuration
#[derive(Debug, Deserialize)]
pub struct NfluxConfig {
    pub interface_names: Vec<String>,
}

// Logging config
#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_level: String,
    pub log_type: String,
}

/// Generic rule for both IPv4 and IPv6
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Rules {
    pub priority: u32,
    pub action: Action,
    pub ports: Vec<u16>,
    pub protocol: Protocol,
    pub log: bool,
    pub description: String,
}

// Top-level configuration structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Nflux {
    pub nflux: NfluxConfig,
    pub logging: LoggingConfig,
    pub ip_rules: HashMap<String, Rules>,
}

impl Nflux {
    // Load the configuration from a file and return the `Nflux` struct
    pub fn load_config() -> Result<Self> {
        let config_file = env::var("NFLUX_CONFIG_FILE_PATH")
            .unwrap_or_else(|_| "/etc/nflux/nflux.toml".to_string());

        let config_content = fs::read_to_string(&config_file)
            .with_context(|| format!("Failed to read configuration file: {}", config_file))?;

        let config: Self = toml::from_str(&config_content)
            .with_context(|| format!("Failed to parse configuration file: {}", config_file))?;

        config.validate()?;

        Ok(config)
    }

    // A separate validation function to ensure correctness
    pub fn validate(&self) -> Result<()> {
        for (ip, rule) in &self.ip_rules {
            if rule.priority == 0 {
                anyhow::bail!("Priority must be greater than 0");
            }
            if !rule.ports.iter().all(|&port| (1..=65535).contains(&port)) {
                anyhow::bail!("Invalid port number in rule for IP: {}", ip);
            }
        }
        Ok(())
    }
}
