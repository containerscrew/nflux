use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::env;
use std::fs;

/// Enum for `action`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")] // Allow "deny" or "allow" in config
pub enum Action {
    Deny,
    Allow,
}

/// Enum for `protocol`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")] // Allow "tcp" or "udp" in config
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
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

/// Configuration for ICMP rules
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct IcmpRules {
    pub action: Action,   // Allow or Deny
    pub protocol: String, // Always "icmp"
}

/// Configuration for MAC-based rules
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MacRule {
    pub action: Action, // Allow or Deny
}

/// General firewall configuration
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Firewall {
    pub interface_names: Vec<String>, // List of interfaces
    pub log_level: String,            // Log level
    pub log_type: String,             // Log type
}

/// Logging configuration
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoggingConfig {
    pub log_denied_packets: bool,
    pub log_allowed_packets: bool,
    pub log_format: String,
    pub log_file: String,
}

/// Top-level configuration structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FirewallConfig {
    pub firewall: Firewall,
    pub ip_rules: HashMap<String, Rules>,
    pub icmp_rules: HashMap<String, IcmpRules>,
    pub mac_rules: HashMap<String, MacRule>,
    pub logging: LoggingConfig,
}

impl FirewallConfig {
    /// Load the configuration from a file, defaulting to `/etc/nflux/nflux.toml` if not specified
    pub fn load() -> Result<Self> {
        let config_file = env::var("NFLUX_CONFIG_FILE_PATH")
            .unwrap_or_else(|_| "/etc/nflux/nflux.toml".to_string());

        let config_content = fs::read_to_string(&config_file)
            .with_context(|| format!("Failed to read configuration file: {}", config_file))?;

        toml::from_str(&config_content)
            .with_context(|| format!("Failed to parse configuration file: {}", config_file))
    }
}
