use std::fs::{read_to_string, write};
use std::path::Path;

use nanologger::info;
use serde::{Deserialize, Serialize};

use crate::error::config::pounce::PounceConfigError;

#[derive(Serialize, Deserialize)]
pub struct PounceConfig {
    pub main       : MainConfig, 
    pub containerd : ContainerdConfig
}

#[derive(Serialize, Deserialize)]
pub struct MainConfig {
    pub node_cfg_file: String
}

#[derive(Serialize, Deserialize)]
pub struct ContainerdConfig {
    pub socket_file: String
}

impl Default for PounceConfig {
    fn default() -> Self {
        Self {
            main: MainConfig {
                node_cfg_file: "node.toml".to_string()
            },
            containerd: ContainerdConfig {
                socket_file: "/run/containerd/containerd.sock".to_string()
            }
        }
    }
}

impl PounceConfig {
    pub fn new() -> Result<Self, PounceConfigError> {
        let path = "config.toml";

        info!("Loading config `{}`", path);

        if !Path::new(path).exists() {
            info!("Creating default `{}` file", path);
            let default = PounceConfig::default();
            let config = toml::to_string_pretty(&default)?;
            write(path, config)
                .map_err(|e| PounceConfigError::WriteToFileFailed(e, path.to_string()))?;
            return Ok(default);
        }

        let text = read_to_string(path)
            .map_err(|e| PounceConfigError::ReadFromFileFailed(e, path.to_string()))?;
        let config = toml::from_str(&text)?;
        info!("Loaded config `{}`", path);
        Ok(config)
    }
}