use serde::Deserialize;
use std::fs;
use tracing::trace;
#[derive(Deserialize)]
pub struct LoggingConfig {
    pub(crate) log_level: String,
    pub(crate) log_type: String,
}
#[derive(Deserialize)]
pub struct Nflux {
    pub(crate) interface_name: String,
}

#[derive(Deserialize)]
pub struct FirewallConfig {
    pub(crate) allowed_ipv4: Vec<String>,
    pub(crate) allowed_ports: Vec<u32>,
    pub(crate) allow_icmp: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub(crate) log: LoggingConfig,
    pub(crate) firewall: FirewallConfig,
    pub(crate) nflux: Nflux,
}

impl Config {
    // This function loads the configuration from the file
    pub(crate) fn load_config(config_file: &str) -> Self {
        let config_content =
            fs::read_to_string(config_file.to_string()).expect("Failed to read configuration file");
        match toml::from_str(&config_content) {
            Ok(config) => {
                trace!("Configuration loaded successfully");
                config
            }
            Err(e) => {
                panic!("Failed to parse configuration file: {}", e);
            }
        }
    }
}
