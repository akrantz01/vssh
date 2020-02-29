use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, read_to_string};
use std::io::prelude::*;
use std::fmt;
use std::path::Path;
use url::{Url, ParseError};

pub enum Error {
    InvalidUrl(ParseError),
    NoToken,
    NoKey,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            Error::NoToken => write!(f, "No token provided"),
            Error::NoKey => write!(f, "Specified key does not exist"),
        }
    }
}

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

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Error> {
        // Validate URL
        match Url::parse(self.server.as_str()) {
            Ok(_) => {},
            Err(e) => {
                return Err(Error::InvalidUrl(e));
            }
        };

        // Ensure non-empty token
        if self.token == "" {
            return Err(Error::NoToken);
        }

        // Ensure key exists
        if !Path::new(&self.default_key).exists() {
            return Err(Error::NoKey);
        }

        Ok(())
    }
}
