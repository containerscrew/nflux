use std::fs;
use nflux::{Action, Nflux, Protocol};
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


// #[test]
// fn test_load_missing_config_file() {
//     std::env::set_var("NFLUX_CONFIG_FILE_PATH", "/nonexistent/path/nflux.toml");

//     let result = Nflux::load_config();

//     // Assert that loading fails
//     assert!(result.is_err());
//     assert!(result
//         .unwrap_err()
//         .to_string()
//         .contains("Failed to read configuration file"));
// }

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
