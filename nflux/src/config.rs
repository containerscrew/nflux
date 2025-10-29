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
#[allow(dead_code)]
pub struct NfluxConfig {
    pub logging: LoggingConfig,
    pub agent: AgentConfig,
}

impl NfluxConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Error reading the config file in '{}': {}", path, e))?;

        let config: NfluxConfig =
            toml::from_str(&contents).map_err(|e| format!("Error parsing TOML content: {}", e))?;

        Ok(config)
    }
}
