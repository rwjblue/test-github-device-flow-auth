use super::errors::DeviceFlowError;

#[cfg(debug_assertions)]
use std::process::Command;

#[cfg(not(debug_assertions))]
use keyring::Entry;

pub fn get_password() -> Result<String, DeviceFlowError> {
    let service = "test-github-device-flow";
    let username = "github_token";

    #[cfg(debug_assertions)]
    {
        let service = format!("{}:debug", service);
        let output = Command::new("security")
            .args([
                "find-generic-password",
                "-a",
                username,
                "-s",
                &service,
                "-w",
            ])
            .output()
            .map_err(|e| DeviceFlowError::Other(e.to_string()))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(DeviceFlowError::Other(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ))
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let service = format!("{}:release", service);
        let entry = Entry::new(&service, username)?;

        match entry.get_password() {
            Ok(token) => Ok(token),
            Err(error) => Err(DeviceFlowError::Keyring(error)),
        }
    }
}

pub fn save_password(token: String) -> Result<String, DeviceFlowError> {
    let username = "github_token";
    let service = "test-github-device-flow";

    #[cfg(debug_assertions)]
    {
        let service = format!("{}:debug", service);

        let output = Command::new("security")
            .args([
                "add-generic-password",
                "-a",
                username,
                "-s",
                &service,
                "-w",
                &token,
                "-U",
            ])
            .output()
            .map_err(|e| DeviceFlowError::Other(e.to_string()))?;

        if output.status.success() {
            Ok(token)
        } else {
            Err(DeviceFlowError::Other(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ))
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let service = format!("{}:release", service);
        let entry = Entry::new(&service, username)?;

        entry.set_password(&token)?;

        Ok(token)
    }
}
