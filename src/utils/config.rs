use serde::Deserialize;
use std::fs;
use toml;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Master {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub api: ApiConfig,
    pub storage: StorageConfig,
    pub master: Master,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub port: u16,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub path: String,
    pub max_size_mb: u64,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        return Ok(config);
    }
}
