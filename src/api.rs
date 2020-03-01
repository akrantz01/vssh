use crate::config::Config;
use crate::errors::ApiError;
use reqwest::{blocking::Client, Method};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

pub struct ApiClient {
    token: String,
    address: String,
    client: Client,
}

impl ApiClient {
    /// Create a HTTP client from the config file
    pub fn from_config(config: Config) -> Self {
        ApiClient {
            token: config.token,
            address: config.server,
            client: Client::new(),
        }
    }

    /// Ensure the configuration is valid by getting the token permissions
    pub fn validate(&self) -> Result<bool, reqwest::Error> {
        let response = self.client.get(&format!("{}/v1/auth/token/lookup-self", self.address).to_string())
            .header("X-Vault-Token", self.token.as_str())
            .send()?;
        Ok(response.status().is_success())
    }

    /// Sign a given public key with the specified role
    pub fn sign(&self, role: String, key: String) -> Result<String, ApiError> {
        let mut body = HashMap::new();
        body.insert("public_key", key);

        let response = self.client.put(&format!("{}/v1/ssh-ca/sign/{}", self.address, role))
            .header("X-Vault-Token", self.token.as_str())
            .json(&body)
            .send()?;

        // Ensure successful
        let status = response.status();
        if status.is_client_error() {
            let error: ErrorResponse = response.json()?;
            Err(self.response_to_error(error))
        } else if status.is_server_error() {
            Err(ApiError::ServerError)
        } else {
            let signed: SignResponse = response.json()?;
            Ok(signed.data.signed_key)
        }
    }

    /// Get a list of roles to sign as
    pub fn list_roles(&self) -> Result<Vec<String>, ApiError> {
        let response = self.client.request(Method::from_str("LIST").unwrap(), &format!("{}/v1/ssh-ca/roles", self.address))
            .header("X-Vault-Token", self.token.as_str())
            .send()?;

        let status = response.status();
        if status.is_client_error() {
            let error: ErrorResponse = response.json()?;
            Err(self.response_to_error(error))
        } else if status.is_server_error() {
            Err(ApiError::ServerError)
        } else {
            let roles: RolesResponse = response.json()?;
            Ok(roles.data.keys)
        }
    }

    /// Convert an error response body to a Rust error
    fn response_to_error(&self, response: ErrorResponse) -> ApiError {
        if response.errors[0].contains("permission denied") {
            ApiError::PermissionDenied
        } else if response.errors[0].contains("missing public_key") {
            ApiError::InvalidPublicKey
        } else if response.errors[0].contains("failed to parse public_key as SSH key") {
            ApiError::InvalidPublicKey
        } else if response.errors[0].contains("Unknown role") {
            ApiError::UnknownRole
        } else {
            ApiError::UnknownError
        }
    }
}

#[derive(Deserialize)]
struct SignResponse {
    pub data: SignData
}

#[derive(Deserialize)]
struct SignData {
    pub signed_key: String
}

#[derive(Deserialize)]
struct RolesResponse {
    pub data: RolesData
}

#[derive(Deserialize)]
struct RolesData {
    pub keys: Vec<String>
}

#[derive(Deserialize)]
struct ErrorResponse {
    pub errors: Vec<String>
}
