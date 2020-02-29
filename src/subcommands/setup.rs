use crate::config::Config;
use std::process::exit;

pub fn interactive() {
    println!("Running interactively");
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

    // Write the configuration to disk
    let config = Config::new(String::from(server), String::from(token), String::from(key), String::from(path), tls);
    match config.write() {
        Ok(_) => println!("Successfully configured"),
        Err(e) => {
            println!("Error configuring: {}", e);
        }
    }
}
