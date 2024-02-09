use super::config::Config;
use super::errors::DeviceFlowError;

// Import `Command` for macOS-specific functionality
#[cfg(target_os = "macos")]
use std::process::Command;

// Use `Entry` from `keyring` for non-macOS platforms
#[cfg(not(target_os = "macos"))]
use keyring::Entry;

pub fn get_password() -> Result<String, DeviceFlowError> {
    let service = "test-github-device-flow";
    let username = Config::load()?.user;

    #[cfg(target_os = "macos")]
    {
        let service = format!("{}:debug", service);
        let output = Command::new("security")
            .args([
                "find-generic-password",
                "-a",
                &username,
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

    #[cfg(not(target_os = "macos"))]
    {
        let service = format!("{}:release", service);
        let entry = Entry::new(&service, &username)?;

        match entry.get_password() {
            Ok(token) => Ok(token),
            Err(error) => Err(DeviceFlowError::Keyring(error)),
        }
    }
}

pub fn save_password(username: String, token: String) -> Result<String, DeviceFlowError> {
    let service = "test-github-device-flow";

    #[cfg(target_os = "macos")]
    {
        let service = format!("{}:debug", service);

        let output = Command::new("security")
            .args([
                "add-generic-password",
                "-a",
                &username,
                "-s",
                &service,
                "-w",
                &token,
                "-U",
            ])
            .output()
            .map_err(|e| DeviceFlowError::Other(e.to_string()))?;

        if !output.status.success() {
            return Err(DeviceFlowError::Other(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let service = format!("{}:release", service);
        let entry = Entry::new(&service, &username)?;

        entry.set_password(&token)?;

        Ok(token)
    }

    let config = Config { user: username };
    config.save()?;

    Ok(token)
}
