use std::io;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] ParseError),
    #[error("Invalid authentication token")]
    InvalidToken,
    #[error("Configuration file does not exist")]
    NonExistentConfigFile,
    #[error("Failed to read from file: {0}")]
    ReadError(#[from] io::Error),
    #[error("Failed to decode JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Failed to decode certificate: {0}")]
    CertificateDecodeError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Server error")]
    ServerError,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Unknown role")]
    UnknownRole,
    #[error("Invalid public key format")]
    InvalidPublicKey,
    #[error("Failed to send request: {0}")]
    SendFailure(#[from] reqwest::Error),
    #[error("An unknown error occurred")]
    UnknownError,
}
