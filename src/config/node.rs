use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_to_string, write};
use std::path::Path;

use nanologger::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NodeConfig {
    pub instance : InstanceConfig,
    pub servers  : HashMap<String, ServerConfig>
}

#[derive(Serialize, Deserialize)]
pub struct InstanceConfig {

}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {

}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            instance: InstanceConfig {  },
            servers: HashMap::new() 
        }
    }
}

impl NodeConfig {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        info!("Loading node config `{}`", path);

        if !Path::new(path).exists() {
            info!("Creating default `{}` file", path);
            let default = NodeConfig::default();
            let config = toml::to_string_pretty(&default)?;
            write(path, config)?;
            return Ok(default);
        }

        let text = read_to_string(path)?;
        let config = toml::from_str(&text)?;
        info!("Loaded node config `{}`", path);
        Ok(config)
    }
}