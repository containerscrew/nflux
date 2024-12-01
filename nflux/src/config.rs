use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct LoggingConfig {
    pub log_level: String,
    pub log_type: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Nflux {
    pub interface_name: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FirewallConfig {
    pub allowed_ipv4: Vec<String>,
    pub allowed_ports: Vec<u32>,
    pub allow_icmp: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub log: LoggingConfig,
    pub firewall: FirewallConfig,
    pub nflux: Nflux,
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
