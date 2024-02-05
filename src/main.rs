use attohttpc::{header, Method, RequestBuilder, StatusCode};
use dialoguer::{Confirm, Input};
use is_terminal::IsTerminal;
use serde_json::Value;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gh_token = env::var("GH_TOKEN").or(env::var("GITHUB_AUTH")).ok();

    let token = match gh_token {
        Some(token) => token,
        None => {
            if std::io::stdout().is_terminal() {
                // We are in an interactive terminal, proceed with GitHub device flow
                get_github_token()?
            } else {
                // Not an interactive terminal, exit or handle accordingly
                eprintln!("GitHub token not found and not in an interactive terminal.");
                std::process::exit(1);
            }
        }
    };

    // Example: Downloading source code, adjust the URL to your needs
    let url = "https://api.github.com/repos/{owner}/{repo}/zipball";
    let response = attohttpc::get(url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "github_downloader")
        .send()?;

    if response.status() == StatusCode::OK {
        // Handle successful download, e.g., save to a file
    } else {
        // Handle errors
    }

    Ok(())
}

fn get_github_token() -> Result<String, Box<dyn std::error::Error>> {
    // Implement the GitHub device flow here to obtain a token
    // This is a placeholder function. You'll need to follow GitHub's documentation
    // to implement the device flow correctly.
    Ok("your_token".into())
}
