//! The `github_device_flow` module provides functionality to authenticate a user via GitHub's device flow.
//! This includes requesting a device code, prompting the user to authorize the application, and polling
//! GitHub for an access token.
//!
//! This module abstracts the complexities involved in the device flow process and provides a simple
//! interface for obtaining an access token, which can then be used for making authenticated requests to
//! the GitHub API.
//!
//! References:
//! - [GitHub Device Flow](https://docs.github.com/en/developers/apps/authorizing-oauth-apps#device-flow)
//! - [Building a CLI with a GitHub App](https://docs.github.com/en/apps/creating-github-apps/writing-code-for-a-github-app/building-a-cli-with-a-github-app)
//!

use attohttpc::StatusCode;
use std::{thread, time::Duration};

use super::errors::DeviceFlowError;
use super::keychain::{get_password, save_password};

const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_TOKEN_SCOPE: &str = "repo";
//const CLIENT_ID: &str = "Iv1.d2cfa8999c68b819";  // LI internal app
const CLIENT_ID: &str = "Iv1.da140951097b6f6a"; // rwjblue (public) app

#[derive(Debug, serde::Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, serde::Serialize)]
struct DeviceCodeRequest {
    client_id: String,
    scope: String,
}

#[derive(Debug, serde::Deserialize)]
struct TokenPollResponse {
    access_token: Option<String>,
    error: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct TokenPollRequest {
    client_id: String,
    device_code: String,
    grant_type: String,
}

/// This struct is used to deserialize the response from the GitHub API
/// for GET /user
#[derive(serde::Deserialize, Debug)]
struct GithubUser {
    login: String,
}

pub fn get_github_token() -> Result<String, DeviceFlowError> {
    match get_password() {
        Ok(token) => Ok(token),
        Err(_) => create_github_token(),
    }
}

/// Initiates the GitHub device flow to obtain an access token.
///
/// This function first requests a device code and instructs the user to authorize the application.
/// After the user has authorized the application by entering the code on GitHub's website,
/// it polls GitHub for an access token at regular intervals as specified by GitHub.
///
/// # Errors
///
/// This function will return an error if any step of the device flow fails, including network errors,
/// errors from GitHub, or if the polling times out.
///
/// # Returns
///
/// On success, returns the access token as a `String`.
///
/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let token = github_device_flow::get_github_token();
///     println!("Access token: {}", token);
///     Ok(())
/// }
/// ```
pub fn create_github_token() -> Result<String, DeviceFlowError> {
    log::info!("Attempting to request device code");
    // Request a device code
    let response = attohttpc::post(GITHUB_DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .form(&DeviceCodeRequest {
            client_id: CLIENT_ID.to_string(),
            scope: GITHUB_TOKEN_SCOPE.to_string(),
        })?
        .send()?;

    if response.status() != StatusCode::OK {
        return Err(DeviceFlowError::Other(format!(
            "Failed to get device code. Status: {:?}",
            response.status()
        )));
    }

    let device_code_response: DeviceCodeResponse = response.json()?;
    log::info!("DeviceCodeResponse: {:?}", device_code_response);

    println!("Open the following URL in your browser and enter the code:");
    println!("Code: {}", device_code_response.user_code);
    println!("Press Enter once you have copied the code above...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if webbrowser::open(&device_code_response.verification_uri).is_err() {
        eprintln!("Failed to open web browser. Please manually open the URL.");
    }

    let token = poll_for_token(device_code_response)?;
    let username = get_username(&token)?;

    save_password(username, token)
}

fn poll_for_token(device_code_response: DeviceCodeResponse) -> Result<String, DeviceFlowError> {
    // Poll the endpoint for the access token
    let poll_interval = Duration::from_secs(device_code_response.interval);
    let expires_in = Duration::from_secs(device_code_response.expires_in);
    let start_time = std::time::Instant::now();

    loop {
        if start_time.elapsed() > expires_in {
            return Err(DeviceFlowError::Other("Token request timed out".into()));
        }

        log::info!(
            "Pinging {} to checkout on device code {}",
            GITHUB_TOKEN_URL,
            device_code_response.device_code.clone()
        );
        let poll_response = attohttpc::post(GITHUB_TOKEN_URL)
            .header("Accept", "application/json")
            .form(&TokenPollRequest {
                client_id: CLIENT_ID.to_string(),
                device_code: device_code_response.device_code.clone(),
                grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
            })?
            .send()?;

        if poll_response.status() == StatusCode::OK {
            let token_response: TokenPollResponse = poll_response.json()?;
            log::info!("TokenPollResponse: {:?}", token_response);

            if let Some(access_token) = token_response.access_token {
                return Ok(access_token);
            }

            match token_response.error {
                Some(ref error) if error == "authorization_pending" => {
                    // this is normal, not _really_ an error condition
                    thread::sleep(poll_interval);
                }
                Some(ref error) if error == "slow_down" => {
                    thread::sleep(poll_interval + Duration::from_secs(5))
                }
                Some(ref error) if error == "expired_token" => {
                    return Err(DeviceFlowError::Other(
                        "Device token expired, please re-run to try again".into(),
                    ));
                }

                Some(error) => {
                    return Err(DeviceFlowError::Other(format!(
                        "Failed to get token: {}",
                        error
                    )));
                }
                None => {
                    return Err(DeviceFlowError::Other("Failed to get token".into()));
                }
            }
        } else {
            return Err(DeviceFlowError::Other("Failed to poll for token".into()));
        }
    }
}

fn get_username(token: &str) -> Result<String, DeviceFlowError> {
    let response = attohttpc::get("https://api.github.com/user")
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "test-github-device-flow/0.1.0")
        .header("Accept", "application/vnd.github+json")
        .send()?;

    if response.status() != StatusCode::OK {
        let error_message = super::utils::log_failed_request(response);

        return Err(DeviceFlowError::Other(error_message));
    }

    let user: GithubUser = response.json()?;

    Ok(user.login)
}
