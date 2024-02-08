# test-github-device-flow-auth

This project serves as a proof of concept, demonstrating the feasibility of
implementing GitHub's device flow authentication in a command-line interface
(CLI) tool, similar to the authentication process used by the official `gh` CLI
tool. It was developed as a test/spike to explore the authentication mechanism
and is not intended for production use.

## Key Features

- Implements GitHub Device Flow for CLI authentication, enabling users to
  securely authenticate with GitHub and obtain an access token.
- Securely stores the access token using the system's keychain, ensuring
  sensitive information is kept safe.
- Provides error handling to manage and report issues related to network
  errors, GitHub responses, or token expiry.
- Includes a configurable setup through YAML files for easy adaptation and
  testing of different scenarios.

## Requirements

In order for this to function, you must have a basic GitHub App installed into
your organizations. The GitHub App must enable the "Device Flow" capability. It
does not have to do anything else though (no webhooks, &c).
