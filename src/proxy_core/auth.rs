use std::{sync::Arc, time::Duration};

use reqwest::{Client, StatusCode};

use crate::{config::Config, dto::auth::AuthResponse};

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    NetworkError(String),
    ParseError(String),
    ServiceError(String),
    // You might add more specific errors as needed
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidToken => write!(f, "Invalid API token"),
            AuthError::NetworkError(e) => {
                write!(f, "Network error communicating with auth service: {}", e)
            }
            AuthError::ParseError(e) => write!(f, "Error parsing auth service response: {}", e),
            AuthError::ServiceError(e) => write!(f, "Auth service error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {}

pub struct AuthClient {
    client: Client,
    path: String,
}

impl AuthClient {
    pub fn new(config: Arc<Config>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(config.http_proxy.timeout))
            .build()
            .unwrap();
        AuthClient {
            client,
            path: config.auth_url.clone(),
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<AuthResponse, AuthError> {
        // let mut headers: HashMap<String, String> = HashMap::new();
        // headers.insert("X-API-KEY", token);
        let response = self
            .client
            .get(self.path.as_str())
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(e.to_string()))?;
        if response.status() == StatusCode::OK {
            let auth_response: AuthResponse = response
                .json()
                .await
                .map_err(|e| AuthError::ParseError(e.to_string()))?;
            Ok(auth_response)
        } else if response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::FORBIDDEN
        {
            // Token is invalid or forbidden by the auth service itself
            Err(AuthError::InvalidToken)
        } else {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "No body".to_string());
            Err(AuthError::ServiceError(format!(
                "Auth service returned status {}: {}",
                status, text
            )))
        }
    }
}
