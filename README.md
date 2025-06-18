# Styx: Custom API Gateway for Cerberus Tokens

Styx is an API Gateway developed in Rust, specifically designed to work with Cerberus tokens. This project provides a robust and flexible solution for managing routes, roles, and granular permissions, all stored directly within the Cerberus tokens. Furthermore, Styx integrates with an external service for token verification and extraction, ensuring centralized and efficient access control.

## Key Features

- __Cerberus Token-Based Authentication & Authorization__: Leverage the security of Cerberus tokens to manage access to your services.
- __Granular Permission Management__: Define detailed roles and permissions directly in your tokens, allowing precise, route-level access control.
- __External Token Verification Service__: Connects to a configurable external service for validating and extracting token information.
- __Flexible Configuration__: Customize Styx's behavior through an easy-to-use configuration file.
- __Comprehensive Logging__: Configurable logging options to monitor gateway traffic and events.

## Project Structure

At its core, Styx efficiently and securely handles and validates Cerberus tokens, routing requests as needed. Project configuration is managed via a `config.lua` file, allowing you to fine-tune various operational parameters.

## Configuration

Styx uses a `config.lua` file for all its configurations. Below is an example configuration and a brief description of each option:

```lua
config = {
    log_file_path = "./var/log/styx",
    database_path = "./lib/routes.db",
    some_option = "valor opcional", -- An example of a customizable option
    log_level = "info", -- Minimum log level (trace, debug, info, warn, error)
    log_to_console = true, -- Enables logging to console
    log_to_file = true, -- Enables logging to a file
    log_format_json = true, -- Formats logs as JSON
    cache_TTL = 36000, -- Cache Time To Live in seconds
    auth_url = "http://localhost:3000/v1/auth/check", -- URL of the Cerberus token verification service
    token_header= "api-token" -- HTTP header where the token is expected
}

http_proxy = {
    port = 3080, -- Port for the main HTTP proxy
    timeout = 30000, -- Proxy request timeout in milliseconds
    address = "0.0.0.0" -- Listen address for the HTTP proxy
}

admin = {
    port = 3082, -- Port for the admin interface
    address = "0.0.0.0" -- Listen address for the admin interface
}
```

__Configuration Options__
- _`config.log_file_path`_: Path where log files will be saved.
- _`config.database_path`_: Path to the database file where routes are stored.
- _`config.some_option`_: An example option you can use to customize Styx.
- _`config.log_level`_: Defines the verbosity of logs (e.g., info, debug, error).
- _`config.log_to_console`_: true to display logs in the console, false to disable.
- _`config.log_to_file`_: true to save logs to a file, false to disable.
- _`config.log_format_json`_: true to format logs as JSON, false for plain text format.
- _`config.cache_TTL`_: Time-to-live in seconds for token information cache.
- _`config.auth_url`_: The URL of the external service responsible for verifying and extracting information from Cerberus or other protocols tokens.
- _`config.token_header`_: The name of the HTTP header where Styx expects to find the Cerberus token in incoming requests.
- _`http_proxy.port`_: The port on which Styx's HTTP proxy will listen for requests.
- _`http_proxy.timeout`_: The maximum time in milliseconds the proxy will wait for a response from backend services.
- _`http_proxy.address`_: The IP address on which Styx's HTTP proxy will listen. 0.0.0.0 listens on all available interfaces.
- _`admin.port`_: The port on which Styx's administration interface will listen.
- _`admin.address`_: The IP address on which Styx's administration interface will listen. 0.0.0.0 listens on all available interfaces.
