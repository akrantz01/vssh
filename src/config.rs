use crate::errors::ConfigError;
use reqwest::Certificate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: String,
    pub token: String,
    pub path: String,
    pub custom_ca: String,
    pub tls: bool,
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub address: String,
    pub role: String,
    pub private_key: String,
    pub options: String,
}

impl Config {
    /// Initializes a new configuration with the provided values.
    pub fn new(server: String, token: String, path: String, custom_ca: String, tls: bool) -> Self {
        Config {
            server,
            token,
            path,
            custom_ca,
            tls,
            profiles: HashMap::new(),
        }
    }

    /// Read the default configuration file.
    /// This is stored at `$HOME/.config/vssh.yml`
    pub fn read_default() -> Result<Self, ConfigError> {
        let mut home = dirs::home_dir().expect("Failed to retrieve user's home directory");
        home.push(".config/vssh.yml");
        Config::read(
            home.as_path()
                .to_str()
                .expect("Failed to convert path to string")
                .to_string(),
        )
    }

    /// Read the specified configuration file.
    pub fn read(path: String) -> Result<Self, ConfigError> {
        if !Path::new(&path).exists() {
            return Err(ConfigError::NonExistentConfigFile);
        }

        let raw: String = read_to_string(path)?;
        let config: Config = serde_json::from_str(&raw)?;
        Ok(config)
    }

    /// Write the currently stored configuration to the default location.
    /// The configuration is stored as YAML.
    pub fn write(&self) -> Result<(), ConfigError> {
        let encoded = serde_json::to_string(self)?;

        let mut home = dirs::home_dir().expect("Failed to retrieve user's home directory");
        home.push(".config/vssh.yml");

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(home.as_path())?;

        file.write_all(encoded.as_bytes())?;
        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate URL
        match Url::parse(self.server.as_str()) {
            Ok(_) => {}
            Err(e) => {
                return Err(ConfigError::InvalidUrl(e));
            }
        };

        // Ensure non-empty token
        if self.token == "" {
            return Err(ConfigError::InvalidToken);
        }

        // Validate custom CA configuration if in use
        if self.custom_ca != "" {
            self.read_certificate()?;
        }

        Ok(())
    }

    /// Read a PEM encoded public certificate
    pub fn read_certificate(&self) -> Result<Certificate, ConfigError> {
        // Ensure exists
        let mut raw_pem = Vec::new();
        File::open(&self.custom_ca)?.read_to_end(&mut raw_pem)?;

        // Parse PEM
        Ok(Certificate::from_pem(&raw_pem)?)
    }
}
