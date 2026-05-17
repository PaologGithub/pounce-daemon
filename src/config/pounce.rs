use std::error::Error;
use std::fs::{read_to_string, write};
use std::path::Path;

use nanologger::info;
use serde::{Deserialize, Serialize};

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
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let path = "config.toml";

        info!("Loading config `{}`", path);

        if !Path::new(path).exists() {
            info!("Creating default `{}` file", path);
            let default = PounceConfig::default();
            let config = toml::to_string_pretty(&default)?;
            write(path, config)?;
            return Ok(default);
        }

        let text = read_to_string(path)?;
        let config = toml::from_str(&text)?;
        info!("Loaded config `{}`", path);
        Ok(config)
    }
}