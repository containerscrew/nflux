use nflux::Config;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_load_valid_config() {
    let config_content = r#"
    [log]
    log_level = "info"
    log_type = "json"

    [nflux]
    interface_name = "wlp2s0"

    [firewall]
    allowed_ports = [22, 80]
    allowed_ipv4 = []
    allow_icmp = false
    "#;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("nflux.toml");
    fs::write(&config_path, config_content).unwrap();

    std::env::set_var("NFLUX_CONFIG_FILE_PATH", config_path.to_str().unwrap());

    let config = Config::load();
    assert_eq!(config.log.log_level, "info");
    assert_eq!(config.log.log_type, "json");
    assert_eq!(config.firewall.allowed_ports, vec![22, 80]);
}

#[test]
fn test_load_missing_config_file() {
    std::env::set_var("NFLUX_CONFIG_FILE_PATH", "/nonexistent/path/nflux.toml");

    let result = std::panic::catch_unwind(|| Config::load());
    assert!(
        result.is_err(),
        "Expected panic when config file is missing"
    );
}

#[test]
fn test_load_invalid_config_format() {
    let config_content = r#"
    [log
    log_level = "info"
    log_type = "json"
    "#; // Missing closing bracket and invalid TOML format

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("nflux.toml");
    fs::write(&config_path, config_content).unwrap();

    std::env::set_var("NFLUX_CONFIG_FILE_PATH", config_path.to_str().unwrap());

    let result = std::panic::catch_unwind(|| Config::load());
    assert!(
        result.is_err(),
        "Expected panic when config file is invalid"
    );
}

// #[test]
// fn test_load_config_default_value_fallback() {
//     // No environment variable set, expect the default path to be used.
//     std::env::remove_var("NFLUX_CONFIG_FILE_PATH");

//     let result = std::panic::catch_unwind(|| Config::load());
//     assert!(result.is_err(), "Expected panic when default config file is missing");
// }

// #[test]
// fn test_load_partial_config() {
//     let config_content = r#"
//     [log]
//     log_level = "warn"

//     [firewall]
//     allowed_ports = [443]
//     "#; // Missing some fields (e.g., log_type)

//     let temp_dir = tempdir().unwrap();
//     let config_path = temp_dir.path().join("nflux.toml");
//     fs::write(&config_path, config_content).unwrap();

//     std::env::set_var("NFLUX_CONFIG_FILE_PATH", config_path.to_str().unwrap());

//     let config = Config::load();
//     assert_eq!(config.log.log_level, "warn");
//     assert_eq!(config.log.log_type, "text"); // Should fallback to the default value
//     assert_eq!(config.firewall.allowed_ports, vec![443]);
// }

// #[test]
// fn test_load_config_with_multiple_allowed_ips() {
//     let config_content = r#"
//     [log]
//     log_level = "debug"
//     log_type = "json"

//     [firewall]
//     allowed_ports = [22, 443]
//     allowed_ipv4 = ["192.168.0.1", "10.0.0.1"]
//     allow_icmp = true
//     "#;

//     let temp_dir = tempdir().unwrap();
//     let config_path = temp_dir.path().join("nflux.toml");
//     fs::write(&config_path, config_content).unwrap();

//     std::env::set_var("NFLUX_CONFIG_FILE_PATH", config_path.to_str().unwrap());

//     let config = Config::load();
//     assert_eq!(config.log.log_level, "debug");
//     assert_eq!(config.log.log_type, "json");
//     assert_eq!(config.firewall.allowed_ports, vec![22, 443]);
//     assert_eq!(config.firewall.allowed_ipv4, vec!["192.168.0.1", "10.0.0.1"]);
//     assert!(config.firewall.allow_icmp);
// }
