use keyring::Entry;

use super::errors::DeviceFlowError;

pub fn get_password() -> Result<String, DeviceFlowError> {
    let service = "test-github-device-flow";
    let username = "github_token";
    let entry = Entry::new(service, username)?;

    match entry.get_password() {
        Ok(token) => Ok(token),
        Err(error) => Err(DeviceFlowError::Keyring(error)),
    }
}

pub fn save_password(token: String) -> Result<String, DeviceFlowError> {
    let service = "test-github-device-flow";
    let username = "github_token";
    let entry = Entry::new(service, username)?;

    entry.set_password(&token)?;

    Ok(token)
}
