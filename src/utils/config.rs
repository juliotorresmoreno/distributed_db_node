use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct Management {
    pub addr: String,       
    pub node_id: String,      
    pub cluster_token: String, 
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub storage: StorageConfig,
    pub management: Management, 
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
