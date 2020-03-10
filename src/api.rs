use crate::config::Config;
use crate::errors::ApiError;
use reqwest::{header, Client, Method};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

pub struct ApiClient {
    address: String,
    client: Client,
}

impl ApiClient {
    /// Create a HTTP client from the config file
    pub fn from_config(config: Config) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-Vault-Token",
            header::HeaderValue::from_str(&config.token)
                .expect("Failed to convert to header value"),
        );

        // Generate custom client
        let client = Client::builder()
            .default_headers(headers)
            .use_native_tls()
            .danger_accept_invalid_certs(config.tls);

        ApiClient {
            address: config.server.clone(),
            client: if config.custom_ca != "" {
                client
                    .add_root_certificate(
                        config
                            .read_certificate()
                            .expect("Failed to read custom certificate"),
                    )
                    .build()
                    .expect("Failed to build API client")
            } else {
                client.build().expect("Failed to build API client")
            },
        }
    }

    /// Ensure the configuration is valid by getting the token permissions
    pub async fn validate(&self) -> Result<bool, reqwest::Error> {
        let response = self
            .client
            .get(&format!("{}/v1/auth/token/lookup-self", self.address).to_string())
            .send()
            .await?;
        Ok(response.status().is_success())
    }

    /// Sign a given public key with the specified role
    pub async fn sign(&self, role: String, key: String) -> Result<String, ApiError> {
        let mut body = HashMap::new();
        body.insert("public_key", key);

        let response = self
            .client
            .put(&format!("{}/v1/ssh-ca/sign/{}", self.address, role))
            .json(&body)
            .send()
            .await?;

        // Ensure successful
        let status = response.status();
        if status.is_client_error() {
            let error: ErrorResponse = response.json().await?;
            Err(self.response_to_error(error))
        } else if status.is_server_error() {
            Err(ApiError::ServerError)
        } else {
            let signed: SignResponse = response.json().await?;
            Ok(signed.data.signed_key)
        }
    }

    /// Get a list of roles to sign as
    pub async fn list_roles(&self) -> Result<Vec<String>, ApiError> {
        let response = self
            .client
            .request(
                Method::from_str("LIST").unwrap(),
                &format!("{}/v1/ssh-ca/roles", self.address),
            )
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() {
            let error: ErrorResponse = response.json().await?;
            Err(self.response_to_error(error))
        } else if status.is_server_error() {
            Err(ApiError::ServerError)
        } else {
            let roles: RolesResponse = response.json().await?;
            Ok(roles.data.keys)
        }
    }

    /// Convert an error response body to a Rust error
    fn response_to_error(&self, response: ErrorResponse) -> ApiError {
        if response.errors[0].contains("permission denied") {
            ApiError::PermissionDenied
        } else if response.errors[0].contains("missing public_key")
            || response.errors[0].contains("failed to parse public_key as SSH key")
        {
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
    pub data: SignData,
}

#[derive(Deserialize)]
struct SignData {
    pub signed_key: String,
}

#[derive(Deserialize)]
struct RolesResponse {
    pub data: RolesData,
}

#[derive(Deserialize)]
struct RolesData {
    pub keys: Vec<String>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    pub errors: Vec<String>,
}
