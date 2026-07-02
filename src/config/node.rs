use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use nanologger::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::config::node::NodeConfigError;

#[derive(Serialize, Deserialize)]
pub struct NodeConfig {
    pub instance : InstanceConfig,
    pub servers  : HashMap<String, ServerConfig>
}

#[derive(Serialize, Deserialize)]
pub struct InstanceConfig {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub uuid: Uuid,
    pub name: String
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
    pub fn new(path: &str) -> Result<Self, NodeConfigError> {
        info!("Loading node config `{}`", path);

        if !Path::new(path).exists() {
            info!("Creating default `{}` file", path);
            let default = NodeConfig::default();
            let config = toml::to_string_pretty(&default)?;
            write(path, config)
                .map_err(|e| NodeConfigError::WriteToFileFailed(e, path.to_string()))?;
            return Ok(default);
        }

        let text = read_to_string(path)
            .map_err(|e| NodeConfigError::ReadFromFileFailed(e, path.to_string()))?;
        let config = toml::from_str(&text)?;
        info!("Loaded node config `{}`", path);
        Ok(config)
    }
}