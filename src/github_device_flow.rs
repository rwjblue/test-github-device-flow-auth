use attohttpc::{header, Method, RequestBuilder, StatusCode};
use serde_json::Value;
use std::{thread, time::Duration};
use webbrowser;

const GITHUB_DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const CLIENT_ID: &str = "your_client_id_here"; // Replace with your GitHub App's client ID

#[derive(serde::Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(serde::Serialize)]
struct DeviceCodeRequest {
    client_id: String,
    scope: String,
}

#[derive(serde::Deserialize)]
struct TokenPollResponse {
    access_token: Option<String>,
    error: Option<String>,
}

#[derive(serde::Serialize)]
struct TokenPollRequest {
    client_id: String,
    device_code: String,
    grant_type: String,
}

pub fn get_github_token() -> Result<String, Box<dyn std::error::Error>> {
    // Request a device code
    let response = attohttpc::post(GITHUB_DEVICE_CODE_URL)
        .form(&DeviceCodeRequest {
            client_id: CLIENT_ID.to_string(),
            scope: "repo".to_string(),
        })?
        .send()?;

    if response.status() != StatusCode::OK {
        return Err("Failed to get device code".into());
    }

    let device_code_response: DeviceCodeResponse = response.json()?;

    println!("Open the following URL in your browser and enter the code:");
    println!("{}", device_code_response.verification_uri);
    println!("Code: {}", device_code_response.user_code);
    println!("Press Enter after you have authorized...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if webbrowser::open(&device_code_response.verification_uri).is_err() {
        eprintln!("Failed to open web browser. Please manually open the URL.");
    }

    // Poll the endpoint for the access token
    let poll_interval = Duration::from_secs(device_code_response.interval);
    let expires_in = Duration::from_secs(device_code_response.expires_in);
    let start_time = std::time::Instant::now();

    loop {
        if start_time.elapsed() > expires_in {
            return Err("Token request timed out".into());
        }

        let poll_response = attohttpc::post(GITHUB_TOKEN_URL)
            .form(&TokenPollRequest {
                client_id: CLIENT_ID.to_string(),
                device_code: device_code_response.device_code.clone(),
                grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
            })?
            .send()?;

        if poll_response.status() == StatusCode::OK {
            let token_response: TokenPollResponse = poll_response.json()?;
            if let Some(access_token) = token_response.access_token {
                return Ok(access_token);
            }

            match token_response.error {
                Some(ref error) if error == "authorization_pending" => {
                    // this is normal, not _really_ an error condition
                }
                // TODO: handle other errors, specifically slow_down
                Some(error) => {
                    return Err(format!("Failed to get token: {}", error).into());
                }
                None => {
                    return Err("Failed to get token".into());
                }
            }
        } else {
            return Err("Failed to poll for token".into());
        }

        thread::sleep(poll_interval);
    }
}
