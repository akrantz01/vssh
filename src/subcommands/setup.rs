use crate::config::Config;
use crate::util::fail;
use std::io::{self, BufRead, Write};

pub fn interactive() {
    // Get the server address
    let server = prompt_default(
        "What server should vssh connect to?",
        String::from("https://127.0.0.1:8200"),
    );

    // Check if using HTTPS
    let using_https = prompt_bool("Are you using HTTPS to connect to the server?", true);

    // Options only for those using HTTPS
    let custom_ca: String;
    let tls = if using_https {
        // Ask if using custom CA and get certificate path
        custom_ca = if prompt_bool("Are you using a custom certificate authority?", false) {
            prompt("What is the path to your CA's public key?")
        } else {
            String::from("")
        };

        // Prompt for TLS verification
        prompt_bool("Do you want to verify TLS certificates when connecting to the server? Answer no if you are using a self-signed certificate.", true)
    } else {
        custom_ca = String::from("");
        true
    };

    // Get the authentication token
    let token = prompt("What token to vssh authenticated to the server with?");

    // Get the path for the SSH secret engine
    let path = prompt_default(
        "What path is the SSH CA located at on the server?",
        String::from("ssh-ca"),
    );

    // Ensure the configuration is valid
    let config = Config::new(server, token, path, custom_ca, tls);
    match config.validate() {
        Ok(_) => {}
        Err(e) => fail(&format!("Invalid configuration: {}", e)),
    }

    // Write the configuration to disk
    match config.write() {
        Ok(_) => leg::success("Successfully configured", None, None),
        Err(e) => fail(&format!("Error configuring: {}", e)),
    }
}

pub fn noninteractive(server: String, tls: bool, token: String, path: String, custom_ca: String) {
    // Ensure each parameter exists
    if server == "" {
        fail("Option '--server' is required when running non-interactively");
    } else if token == "" {
        fail("Option '--token' is required when running non-interactively");
    } else if path == "" {
        fail("Option '--path' is required when running non-interactively");
    }

    // Ensure the configuration is valid
    let config = Config::new(server, token, path, custom_ca, tls);
    match config.validate() {
        Ok(_) => {}
        Err(e) => fail(&format!("Invalid configuration: {}", e)),
    }

    // Write the configuration to disk
    match config.write() {
        Ok(_) => leg::success("Successfully configured", None, None),
        Err(e) => fail(&format!("Error configuring: {}", e)),
    }
}

/// Read a line from stdin with a given prompt.
fn prompt(prompt: &'static str) -> String {
    // Display the prompt
    print!("{} ", prompt);
    io::stdout().flush().expect("Failed to read from stdout");

    // Get the value
    read_line()
}

/// Read a line from stdin with a given prompt and default.
/// The default option provided will be used if no input is given.
fn prompt_default(prompt: &'static str, default: String) -> String {
    // Display prompt
    print!("{} [{}] ", prompt, default);
    io::stdout().flush().expect("Failed to flush stdout");

    // Get the value
    let line = read_line();

    // Use default if empty
    if line == "" {
        default
    } else {
        line
    }
}

/// Read a boolean with a yes or no prompt from stdin.
fn prompt_bool(prompt: &'static str, default: bool) -> bool {
    // Display prompt
    if default {
        print!("{} [Y/n] ", prompt);
    } else {
        print!("{} [y/N] ", prompt);
    }
    io::stdout().flush().expect("Failed to flush stdout");

    // Get the value
    let line = read_line();

    // Convert string response to boolean
    if line == "y" || line == "Y" || line == "yes" || line == "Yes" {
        true
    } else if line == "n" || line == "N" || line == "no" || line == "No" {
        false
    } else {
        default
    }
}

/// Read a line from stdin
fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .lock()
        .read_line(&mut line)
        .expect("Failed to read from stdin");
    line.trim().to_string()
}
