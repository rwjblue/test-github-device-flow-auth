use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceFlowError {
    #[error("An unexpected error occurred: {0}")]
    Other(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] attohttpc::Error),

    #[cfg(not(target_os = "macos"))]
    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("Yaml parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Could not find the home directory")]
    HomeDirNotFound(),

    #[error("Config file not found: {0}")]
    MissingConfigFile(String),
}
