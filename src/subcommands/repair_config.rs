use crate::config::{Config, Profile};
use crate::util::fail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct RepairableConfig {
    server: Option<String>,
    token: Option<String>,
    path: Option<String>,
    custom_ca: Option<String>,
    tls: Option<bool>,
    profiles: Option<HashMap<String, RepariableProfile>>,
}

#[derive(Serialize, Deserialize)]
struct RepariableProfile {
    username: Option<String>,
    address: Option<String>,
    role: Option<String>,
    private_key: Option<String>,
    public_key: Option<String>,
    options: Option<String>,
}

pub fn repair_config(path: Option<String>) {
    // Use provided file or default
    let path = if let Some(path) = path {
        path
    } else {
        let mut home = dirs::home_dir().expect("Failed to retrieve user's home directory");
        home.push(".config/vssh.json");
        home.as_path()
            .to_str()
            .expect("Failed to convert path to string")
            .to_string()
    };

    // Read in raw configuration file
    let raw = match read_to_string(&path) {
        Ok(raw) => raw,
        Err(e) => fail(&format!("Failed to read configuration file: {}", e)),
    };

    // Attempt to parse JSON
    let unrepaired_config = match serde_json::from_str::<RepairableConfig>(&raw) {
        Ok(cfg) => cfg,
        Err(e) => fail(&format!(
            "Provided configuration file has unparsable JSON: {}",
            e
        )),
    };

    // Create new config with defaults for non-existent fields
    let mut config = Config::new(
        unrepaired_config
            .server
            .unwrap_or(String::from("https://127.0.0.1:8200")),
        unrepaired_config.token.unwrap_or_default(),
        unrepaired_config.path.unwrap_or(String::from("ssh-ca")),
        unrepaired_config.custom_ca.unwrap_or_default(),
        unrepaired_config.tls.unwrap_or(true),
    );

    // Attempt to repair profiles
    if let Some(profiles) = unrepaired_config.profiles {
        for (name, profile) in profiles {
            config.profiles.insert(
                name,
                Profile {
                    username: profile.username.unwrap_or(whoami::username()),
                    address: profile.address.unwrap_or_default(),
                    role: profile.role.unwrap_or_default(),
                    private_key: profile.private_key,
                    public_key: profile.public_key,
                    options: profile.options.unwrap_or_default(),
                },
            );
        }
    }

    // Validate configuration file
    match config.validate() {
        Ok(_) => {}
        Err(e) => fail(&format!(
            "Invalid configuration values, run vssh setup to reconfigure: {}",
            e
        )),
    }

    // Serialize the configuration
    let encoded = match serde_json::to_string(&config) {
        Ok(enc) => enc,
        Err(e) => fail(&format!("Failed to encode configuration as JSON: {}", e)),
    };

    // Open output file
    let mut file = match OpenOptions::new().write(true).truncate(true).open(&path) {
        Ok(f) => f,
        Err(e) => fail(&format!(
            "Failed to open configuration file for writing: {}",
            e
        )),
    };

    // Write configuration data
    match file.write_all(encoded.as_bytes()) {
        Ok(_) => {}
        Err(e) => fail(&format!("Failed to write configuration: {}", e)),
    }

    leg::success(
        &format!("Successfully repaired configuration at '{}'", path),
        None,
        None,
    );
}
