use attohttpc::{header, Method, RequestBuilder, StatusCode};
use clap::Parser;
use dialoguer::{Confirm, Input};
use is_terminal::IsTerminal;
use serde_json::Value;
use std::env;

mod github_device_flow;
use github_device_flow::get_github_token;

/// Downloads source code from a specified GitHub repository.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Specifies the 'org/repo' from GitHub to download
    #[clap(value_parser)]
    repo: String,

    /// GitHub token for authentication. If not provided, the program will check environment variables or initiate the GitHub device flow.
    #[clap(short, long, value_parser)]
    token: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Splitting the 'org/repo' argument into separate variables
    let parts: Vec<&str> = args.repo.split('/').collect();
    if parts.len() != 2 {
        eprintln!("The repo argument must be in the format 'org/repo'");
        return Err("Invalid repo format".into());
    }
    let (owner, repo) = (parts[0], parts[1]);

    println!("Organization: {}, Repository: {}", owner, repo);

    let token = match args.token {
        Some(token) => token,
        None => match std::env::var("GH_TOKEN").or_else(|_| std::env::var("GITHUB_AUTH")) {
            Ok(token) => token,
            Err(_) => {
                if std::io::stdout().is_terminal() {
                    // We are in an interactive terminal, proceed with GitHub device flow
                    get_github_token()?
                } else {
                    eprintln!("GitHub token not provided and not in an interactive terminal.");
                    std::process::exit(1);
                }
            }
        },
    };

    // Example: Downloading source code, adjust the URL to your needs
    let url = format!("https://api.github.com/repos/{}/{}/zipball", owner, repo);
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
