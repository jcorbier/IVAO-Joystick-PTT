use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub key: String,
    #[serde(default)]
    pub vatsim_compat: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            key: "F12".to_string(),
            vatsim_compat: false,
        }
    }
}

pub fn load_config() -> Config {
    // X-Plane 12 CWD is the root X-Plane folder
    let config_path = Path::new("Output/preferences/ivao_ptt.toml");

    if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => return config,
                Err(e) => xdebug!("Failed to parse config: {}", e),
            },
            Err(e) => xdebug!("Failed to read config file: {}", e),
        }
    }

    // If we are here, either file didn't exist or failed to load.
    // Write default config.
    let default_config = Config::default();
    if let Ok(content) = toml::to_string_pretty(&default_config) {
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Err(e) = fs::write(config_path, content) {
            xdebug!("Failed to write default config: {}", e);
        }
    }

    default_config
}
