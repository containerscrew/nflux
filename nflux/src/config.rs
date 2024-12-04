use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;

// Enum for `action`
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_temp_config(content: &str) -> TempDir {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("nflux.toml");
        fs::write(&config_path, content).unwrap();

        std::env::set_var("NFLUX_CONFIG_FILE_PATH", config_path.to_str().unwrap());

        temp_dir
    }

    #[test]
    fn test_load_valid_config() {
        let config_content = r#"
        [nflux]
        interface_names = ["eth0", "wlan0"]

        [logging]
        log_level = "debug"
        log_type = "json"

        [ip_rules]
        "192.168.0.1" = { priority = 1, action = "allow", ports = [22], protocol = "tcp", log = true, description = "SSH rule" }
        "#;

        let _temp_dir = setup_temp_config(config_content);

        let config = Nflux::load_config().unwrap();

        // Assertions
        assert_eq!(config.nflux.interface_names, vec!["eth0", "wlan0"]);
        assert_eq!(config.logging.log_level, "debug");
        assert_eq!(config.logging.log_type, "json");

        let rule = config.ip_rules.get("192.168.0.1").unwrap();
        assert_eq!(rule.priority, 1);
        assert_eq!(rule.action, Action::Allow);
        assert_eq!(rule.ports, vec![22]);
        assert_eq!(rule.protocol, Protocol::Tcp);
        assert_eq!(rule.log, true);
        assert_eq!(rule.description, "SSH rule");
    }

    #[test]
    fn test_load_missing_config_file() {
        std::env::set_var("NFLUX_CONFIG_FILE_PATH", "/nonexistent/path/nflux.toml");

        let result = Nflux::load_config();

        // Assert that loading fails
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to read configuration file"));
    }

    // #[test]
    // fn test_load_invalid_config_format() {
    //     let invalid_config_content = "invalid: [toml";

    //     setup_temp_config(invalid_config_content);

    //     let result = Nflux::load_config();

    //     // Assert that loading fails due to parse error
    //     assert!(result.is_err());
    //     assert!(result
    //         .unwrap_err()
    //         .to_string()
    //         .contains("Failed to parse configuration file"));
    // }
}
