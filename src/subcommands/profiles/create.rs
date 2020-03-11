use crate::config::{Config, Profile};
use std::process::exit;

pub fn create(
    name: String,
    username: String,
    address: String,
    role: String,
    private_key: String,
    options: String,
    mut config: Config,
) {
    // Get default username if not provided
    let username = if username == "" {
        whoami::username()
    } else {
        username
    };

    // Ensure profile does not exist
    if config.profiles.contains_key(&name) {
        println!("Profile '{}' already exists", name);
        exit(1);
    }

    // Add profile to configuration
    config.profiles.insert(
        name.clone(),
        Profile {
            username,
            address,
            role,
            private_key,
            options,
        },
    );

    // Write to file
    match config.write() {
        Ok(_) => println!("Successfully created profile '{}'", name),
        Err(e) => {
            println!("Failed to write to configuration file: {}", e);
            exit(1);
        }
    }
}
