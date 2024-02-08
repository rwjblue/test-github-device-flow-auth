use attohttpc::StatusCode;
use clap::Parser;
use is_terminal::IsTerminal;
use std::fs::File;
use std::io::Write;

mod errors;
mod github_device_flow;
mod keychain;

use errors::DeviceFlowError;
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

fn main() -> Result<(), DeviceFlowError> {
    env_logger::init();

    let args = Args::parse();

    // Splitting the 'org/repo' argument into separate variables
    let parts: Vec<&str> = args.repo.split('/').collect();
    if parts.len() != 2 {
        eprintln!("The repo argument must be in the format 'org/repo'");
        return Err(DeviceFlowError::Other("Invalid repo format".into()));
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

    log::info!("Token to use: {}", token);

    // Example: Downloading source code, adjust the URL to your needs
    let url = format!("https://api.github.com/repos/{}/{}/tarball", owner, repo);
    let response = attohttpc::get(url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "test-github-device-flow/0.1.0")
        .header("Accept", "application/vnd.github+json)")
        .send()?;

    if response.status() == StatusCode::OK {
        let content = response.bytes()?;
        let mut file = File::create("downloaded_repo.tar.gz")?;
        file.write_all(&content)?;

        println!("Repository zip file has been saved as downloaded_repo.tar.gz.");
    } else {
        // If the status code indicates an error, print the status code, error message, headers, and body
        let status_code = response.status();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| format!("\t{}: {:?}", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        // Attempt to read the response body. Note: This consumes the response.
        let body = response
            .text()
            .unwrap_or_else(|_| "Failed to read body".into());

        let error_message = format!(
            "Failed to download repository: HTTP Status {}\nHeaders:\n{}\nBody:\n\t{}",
            status_code, headers, body
        );

        // Emit the detailed error to stderr and log
        eprintln!("{}", error_message);
        log::error!("{}", error_message);

        std::process::exit(1);
    }

    Ok(())
}
