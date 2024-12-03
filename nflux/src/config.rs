use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;

// Enum to restrict `action` values
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")] // Allows "deny" and "allow" as lowercase in TOML
pub enum Action {
    Allow,
    Deny,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")] // Allows "deny" and "allow" as lowercase in TOML
pub enum Protocol {
    Tcp,
    Udp,
}

#[derive(Deserialize, Debug)]
pub struct FirewallGlobalConfig {
    pub icmp_enabled: bool,
    pub interface_name: String,
    pub log_level: String,
    pub log_type: String,
}

#[derive(Deserialize, Debug)]
pub struct FirewallIpv4Rules {
    pub action: Action,
    pub ports: Vec<u32>,
    pub protocol: Protocol,
}

#[derive(Deserialize, Debug)]
pub struct FirewallIpv6Rules {
    pub action: Action,
    pub ports: Vec<u32>,
    pub protocol: Protocol,
}

#[derive(Deserialize, Debug)]
pub struct IcmpRules {
    pub action: Action,
}

#[derive(Deserialize, Debug)]
pub struct FirewallConfig {
    pub firewall: FirewallGlobalConfig,
    pub ipv4_rules: HashMap<String, FirewallIpv4Rules>,
    pub ipv6_rules: HashMap<String, FirewallIpv6Rules>,
    pub icmp_rules: HashMap<String, IcmpRules>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub config: FirewallConfig,
}

impl Config {
    /// Load the configuration from a file, defaulting to `/etc/nflux/nflux.toml` if not specified
    pub fn load() -> Self {
        let config_file = env::var("NFLUX_CONFIG_FILE_PATH")
            .unwrap_or_else(|_| "/etc/nflux/nflux.toml".to_string());

        let config_content = match fs::read_to_string(&config_file) {
            Ok(content) => content,
            Err(e) => {
                panic!("Failed to read configuration file {}: {}", config_file, e);
            }
        };

        match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(e) => {
                panic!("Failed to parse configuration file {}: {}", config_file, e);
            }
        }
    }
}
