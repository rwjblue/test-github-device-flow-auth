pub(crate) fn log_failed_request(response: attohttpc::Response) -> String {
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

    log::error!("{}", error_message);

    error_message
}
