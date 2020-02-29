use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, read_to_string};
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: String,
    pub token: String,
    pub default_key: String,
    pub path: String,
    pub tls: bool,
}

impl Config {
    /// Initializes a new configuration with the provided values.
    pub fn new(server: String, token: String, default_key: String, path: String, tls: bool) -> Self {
        Config {
            server,
            token,
            default_key,
            path,
            tls,
        }
    }

    /// Read the default configuration file.
    /// This is stored at `$HOME/.config/vssh.yml`
    pub fn read_default() -> Result<Self, Box<dyn std::error::Error + 'static>> {
        Config::read(String::from("~/.config/vssh.yml"))
    }

    /// Read the specified configuration file.
    pub fn read(path: String) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let raw: String = read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&raw)?;
        Ok(config)
    }

    /// Write the currently stored configuration to the default location.
    /// The configuration is stored as YAML.
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let encoded = serde_yaml::to_string(self)?;

        let mut home = dirs::home_dir().expect("Failed to retrieve user's home directory");
        home.push(".config/vssh.yml");

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(home.as_path())?;
        
        file.write_all(encoded.as_bytes())?;
        Ok(())
    }
}
