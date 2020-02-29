use std::{error, fmt, io};
use url::ParseError;

#[derive(Debug)]
pub enum ConfigError {
    InvalidUrl(ParseError),
    InvalidToken,
    InvalidDefaultKey,
    NonExistentConfigFile,
    ReadError(io::Error),
    YamlError(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
            ConfigError::InvalidToken => write!(f, "Invalid authentication token"),
            ConfigError::InvalidDefaultKey => write!(f, "Invalid default key"),
            ConfigError::NonExistentConfigFile => write!(f, "Configuration file does not exist"),
            ConfigError::ReadError(e) => write!(f, "Failed to read from file: {}", e),
            ConfigError::YamlError(e) => write!(f, "Failed to decode YAML: {}", e),
        }
    }
}

impl error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::ReadError(error)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::YamlError(error)
    }
}
