use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::errors::DeviceFlowError;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub user: String,
}

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

impl Config {
    pub fn load() -> Result<Self, DeviceFlowError> {
        let path = Config::get_config_path()?;

        if !path.exists() {
            return Err(DeviceFlowError::MissingConfigFile(
                path.to_string_lossy().to_string(),
            ));
        }

        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), DeviceFlowError> {
        let path = Config::get_config_path()?;
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf, DeviceFlowError> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join(PROJECT_NAME);
            fs::create_dir_all(&config_dir)?;
            Ok(config_dir.join("config.yml"))
        } else {
            Err(DeviceFlowError::HomeDirNotFound())
        }
    }
}
