use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

// Enum for `action`
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Deny,
    Allow,
}

// Enum for `protocol`
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

// Enum for `is enabled`
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IsEnabled {
    True,
    False,
}

#[derive(Debug, Deserialize)]
pub struct Firewall {
    pub enabled: IsEnabled,
    pub interfaces: Vec<String>,
    pub icmp_ping: IsEnabled,
    pub rules: HashMap<String, FirewallRules>,
}

#[derive(Debug, Deserialize)]
pub struct Egress {
    pub enabled: IsEnabled,
    pub interfaces: Vec<String>,
    #[allow(dead_code)]
    pub log_private_connections: IsEnabled,
}

// Generic rule for both IPv4 and IPv6
#[derive(Debug, Deserialize)]
pub struct FirewallRules {
    pub priority: u32,
    pub action: Action,
    pub ports: Vec<u16>,
    pub protocol: Protocol,
    #[allow(dead_code)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub log_level: String,
    pub log_type: String,
}
// Top-level configuration structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Nflux {
    pub logging: LoggingConfig,
    pub firewall: Firewall,
    pub egress: Egress,
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
        let mut priorities: HashSet<u32> = HashSet::new();

        for (ip, rule) in &self.firewall.rules {
            // Ensure priority is greater than 0
            if rule.priority == 0 {
                anyhow::bail!("Priority must be greater than 0 for rule: {}", ip);
            }

            // Ensure port numbers are within the valid range
            if !rule.ports.iter().all(|&port| (1..=65535).contains(&port)) {
                anyhow::bail!("Invalid port number in rule for IP: {}. Allowed ports: 1-65535", ip);
            }

            // Check for duplicate priorities
            if !priorities.insert(rule.priority) {
                anyhow::bail!(
                    "Duplicate priority found: {} in rule for IP: {}",
                    rule.priority,
                    ip
                );
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
        interface_name = "wlan0"
        icmp_ping = "true"

        [logging]
        log_level = "debug"
        log_type = "json"


        [ip_rules]
        "192.168.0.1" = { priority = 1, action = "allow", ports = [22], protocol = "tcp", log = true, description = "SSH rule" }
        "#;

        let _temp_dir = setup_temp_config(config_content);

        let config = Nflux::load_config().unwrap();

        // Assertions
        assert_eq!(config.firewall.interfaces, ["wlan0", "proton0"]);
        assert_eq!(config.firewall.icmp_ping, IsEnabled::True);
        assert_eq!(config.logging.log_level, "debug");
        assert_eq!(config.logging.log_type, "json");

        let rule = config.firewall.rules.get("192.168.0.1").unwrap();
        assert_eq!(rule.priority, 1);
        assert_eq!(rule.action, Action::Allow);
        assert_eq!(rule.ports, vec![22]);
        assert_eq!(rule.protocol, Protocol::Tcp);
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

    #[test]
    fn test_duplicate_priority() {
        let config_content = r#"
        [nflux]
        interface_name = "wlan0"
        icmp_ping = "true"

        [logging]
        log_level = "debug"
        log_type = "json"

        [icmp_rules]
        # Rules for ICMP traffic
        "192.168.0.0/24" = { action = "deny", protocol = "icmp" }

        [ip_rules]
        "192.168.0.1" = { priority = 1, action = "allow", ports = [22], protocol = "tcp", log = true, description = "SSH rule" }
        "192.168.0.4" = { priority = 1, action = "allow", ports = [80], protocol = "tcp", log = true, description = "Nginx rule" }
        "#;

        let _temp_dir = setup_temp_config(config_content);

        let config = Nflux::load_config();

        // Check that the configuration loading fails due to duplicate priorities
        assert!(
            config.is_err(),
            "Expected duplicate priorities to cause an error"
        );
    }
}
