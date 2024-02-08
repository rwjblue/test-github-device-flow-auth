use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceFlowError {
    #[error("An unexpected error occurred: {0}")]
    Other(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] attohttpc::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}
