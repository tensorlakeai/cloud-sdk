//! HTTP client that interacts with the Tensorlake Cloud API.
use reqwest::{
    Request, Response, StatusCode,
    header::{HeaderMap, InvalidHeaderValue},
};
use std::result::Result;

use crate::error::SdkError;

/// HTTP client that interacts with the Tensorlake Cloud API.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Create a new SDK client.
    pub fn new(base_url: &str, bearer_token: &str) -> Result<Self, SdkError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", bearer_token)
                .parse()
                .map_err(|e: InvalidHeaderValue| SdkError::InvalidHeaderValue(e.to_string()))?,
        );

        let client = reqwest::Client::builder()
            .user_agent(format!(
                "Tensorlake Cloud SDK/{}",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers(headers)
            .build()?;

        Ok(Self {
            base_url: base_url.to_string(),
            client,
        })
    }

    /// Execute an HTTP request.
    pub async fn execute(&self, request: Request) -> Result<Response, SdkError> {
        let response = self.client.execute(request).await?;
        self.handle_response(response).await
    }

    pub fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client.request(method, self.base_url.clone() + path)
    }

    /// Helper function to handle HTTP responses and convert status codes to appropriate errors
    async fn handle_response(
        &self,
        response: reqwest::Response,
    ) -> Result<reqwest::Response, SdkError> {
        let status = response.status();

        match status {
            StatusCode::UNAUTHORIZED => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unauthorized".to_string());
                Err(SdkError::Authentication(message))
            }
            StatusCode::FORBIDDEN => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Forbidden".to_string());
                Err(SdkError::Authorization(message))
            }
            status if status.is_server_error() => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Server error".to_string());
                Err(SdkError::ServerError { status, message })
            }
            status if !status.is_success() => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Request failed".to_string());
                Err(SdkError::ServerError { status, message })
            }
            _ => Ok(response),
        }
    }
}
