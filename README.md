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

## Device Flow Overview

The way the GitHub device flow system works is _basically_ that you create a
GitHub App that has "Device Flow" enabled on it. That provides you with a
client ID that can be used for API requests (e.g. the device flow).

The high level overview of that device login flow is:

- `POST https://github.com//login/device/code` -- This receives the client ID
  for your GitHub App, and provides you with a "device code" and a verification
  URL
- Provide the device code to the user (so that they can finish the
  authorization in later steps)
- Launch a web browser to the provided verification URL
- The user fills in the device code in their browser
- While waiting for the user to fill in their browser, the binary polls
  `https://github.com/login/oauth/access_token` (providing it the client ID and
  the device ID)
- Once the user completes the auth in the browser, the polling endpoint returns
  the token to the user

## Requirements

In order for this to function, you must have a basic GitHub App installed into
your organizations. The GitHub App must enable the "Device Flow" capability. It
does not have to do anything else though (no webhooks, &c).
