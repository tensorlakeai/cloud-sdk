//! HTTP client that interacts with the Tensorlake Cloud API.
use reqwest::{
    Request, Response, StatusCode,
    header::{HeaderMap, InvalidHeaderValue},
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware};
use std::result::Result;

use crate::error::SdkError;

/// HTTP client that interacts with the Tensorlake Cloud API.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    bearer_token: String,
    client: ClientWithMiddleware,
}

impl Client {
    /// Create a new SDK client without any middleware.
    pub fn new(base_url: &str, bearer_token: &str) -> Result<Self, SdkError> {
        let base_client = new_base_client(bearer_token)?;
        let client = ClientBuilder::new(base_client).build();

        Ok(Self {
            base_url: base_url.to_string(),
            bearer_token: bearer_token.to_string(),
            client,
        })
    }

    /// Create a new client with additional middleware.
    ///
    /// This allows users to inject custom middleware such as VCR recording/playback
    /// or other request/response interceptors.
    pub fn with_middleware<M>(self, middleware: M) -> Result<Self, SdkError>
    where
        M: Middleware + 'static,
    {
        let base_client = new_base_client(&self.bearer_token)?;
        let client = ClientBuilder::new(base_client).with(middleware).build();

        Ok(Self { client, ..self })
    }

    /// Execute an HTTP request.
    pub async fn execute(&self, request: Request) -> Result<Response, SdkError> {
        let response = self.client.execute(request).await?;
        self.handle_response(response).await
    }

    pub fn request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> reqwest_middleware::RequestBuilder {
        self.client.request(method, self.base_url.clone() + path)
    }

    pub fn build_multipart_request(
        &self,
        method: reqwest::Method,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::Request, SdkError> {
        Ok(self
            .client
            .request(method, self.base_url.clone() + path)
            .multipart(form)
            .build()?)
    }

    pub fn build_json_request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<reqwest::Request, SdkError> {
        Ok(self
            .client
            .request(method, self.base_url.clone() + path)
            .json(body)
            .build()?)
    }

    pub fn base_request(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> reqwest_middleware::RequestBuilder {
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

fn new_base_client(bearer_token: &str) -> Result<reqwest::Client, SdkError> {
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
    Ok(client)
}
