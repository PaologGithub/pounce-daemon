use std::error::Error;
use std::fs::{read_to_string, write};
use std::path::Path;

use nanologger::info;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PounceConfig {
    pub containerd: ContainerDConfig
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ContainerDConfig {
    pub socket_file: String
}

impl Default for PounceConfig {
    fn default() -> Self {
        Self {
            containerd: ContainerDConfig {
                socket_file: "/run/containerd/containerd.sock".to_string()
            }
        }
    }
}

impl PounceConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let path = "config.toml";

        info!("Loading config under `{}`", path);

        if !Path::new(path).exists() {
            info!("Config not existing; creating it");
            let default = PounceConfig::default();
            let toml_config = toml::to_string_pretty(&default)?;
            write(path, toml_config)?;
            return Ok(default);
        }

        let text = read_to_string(path)?;
        let config = toml::from_str(&text)?;
        info!("Loaded config");
        Ok(config)
    }
}