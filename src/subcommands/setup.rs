use crate::config::Config;
use std::io::{self, BufRead, Write};
use std::process::exit;

pub fn interactive() {
    // Get the server address
    let server = prompt_default("What server should vssh connect to?", String::from("https://127.0.0.1:8200"));

    // Determine whether TLS should be used
    let tls = prompt_bool("Should TLS be use to connect to he server?", true);

    // Get the authentication token
    let token = prompt("What token to vssh authenticated to the server with?");

    // Get the path for the SSH secret engine
    let path = prompt_default("What path is the SSH CA located at on the server?", String::from("ssh-ca"));

    // Get the default key to sign
    let mut home = dirs::home_dir().expect("Failed to retrieve home directory");
    home.push(".ssh/id_rsa");
    let default_key = prompt_default("What should the default key to sign be?", format!("{}", home.as_path().display()));

    // Ensure the configuration is valid
    let config = Config::new(server, token, default_key, path, tls);
    match config.validate() {
        Ok(_) => {},
        Err(e) => {
            println!("Invalid configuration: {}", e);
            exit(1);
        }
    }

    // Write the configuration to disk
    match config.write() {
        Ok(_) => println!("Successfully configured!"),
        Err(e) => println!("Error configuring: {}", e)
    }
}

pub fn noninteractive<'a>(server: &'a str, tls: bool, token: &'a str, key: &'a str, path: &'a str) {
    // Ensure each parameter exists
    if server == "" {
        println!("Option '--server' is required when running non-interactively");
        exit(1);
    } else if token == "" {
        println!("Option '--token' is required when running non-interactively");
        exit(1);
    } else if key == "" {
        println!("Option '--default-key' is required when running non-interactively");
        exit(1);
    } else if path == "" {
        println!("Option '--path' is required when running non-interactively");
        exit(1);
    }

    // Ensure the configuration is valid
    let config = Config::new(String::from(server), String::from(token), String::from(key), String::from(path), tls);
    match config.validate() {
        Ok(_) => {},
        Err(e) => {
            println!("Invalid configuration: {}", e);
            exit(1);
        }
    }

    // Write the configuration to disk
    match config.write() {
        Ok(_) => println!("Successfully configured"),
        Err(e) => println!("Error configuring: {}", e)
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
    io::stdin().lock().read_line(&mut line).expect("Failed to read from stdin");
    line.trim().to_string()
}
