//! HTTP client that interacts with the Tensorlake Cloud API.
use reqwest::{
    Request, Response, StatusCode,
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
};
use reqwest_middleware::{ClientBuilder as ReqwestClientBuilder, ClientWithMiddleware, Middleware};
use std::{result::Result, sync::Arc};

use crate::error::SdkError;

/// HTTP client that interacts with the Tensorlake Cloud API.
#[derive(Clone)]
pub struct Client {
    base_url: String,
    client: ClientWithMiddleware,
}

/// Builder for creating a [`Client`] with a fluent API.
///
/// The base URL is required, while bearer token, middlewares, and scope are optional.
pub struct ClientBuilder {
    base_url: String,
    bearer_token: Option<String>,
    middlewares: Vec<Arc<dyn Middleware + 'static>>,
    organization_id: Option<String>,
    project_id: Option<String>,
}

impl ClientBuilder {
    /// Create a new [`ClientBuilder`] with the specified base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            bearer_token: None,
            middlewares: Vec::new(),
            organization_id: None,
            project_id: None,
        }
    }

    /// Set the bearer token for authentication.
    pub fn bearer_token(mut self, token: &str) -> Self {
        self.bearer_token = Some(token.to_string());
        self
    }

    /// Add middleware to the client.
    pub fn middleware<M>(mut self, middleware: M) -> Self
    where
        M: Middleware + 'static,
    {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    /// Add multiple middlewares to the client.
    pub fn middlewares(mut self, middlewares: Vec<Arc<dyn Middleware + 'static>>) -> Self {
        self.middlewares = middlewares;
        self
    }

    /// Set the organization and project scope.
    pub fn scope(mut self, organization_id: &str, project_id: &str) -> Self {
        self.organization_id = Some(organization_id.to_string());
        self.project_id = Some(project_id.to_string());
        self
    }

    /// Build the [`Client`].
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created or configured.
    pub fn build(self) -> Result<Client, SdkError> {
        let mut default_headers = HeaderMap::new();

        // Add bearer token if provided
        if let Some(token) = &self.bearer_token {
            default_headers = new_default_headers(token)?;
        }

        // Add scope headers if provided
        if let Some(org_id) = &self.organization_id {
            default_headers.insert("X-Tensorlake-Organization-Id", str_to_header_value(org_id)?);
        }
        if let Some(project_id) = &self.project_id {
            default_headers.insert("X-Tensorlake-Project-Id", str_to_header_value(project_id)?);
        }

        let base_client = new_base_client(&default_headers)?;
        let mut builder = ReqwestClientBuilder::new(base_client);

        for middleware in &self.middlewares {
            builder = builder.with_arc(middleware.clone());
        }

        let client = builder.build();

        Ok(Client {
            base_url: self.base_url,
            client,
        })
    }
}

impl Client {
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
        self.client
            .request(method, self.base_url.clone() + path)
            .multipart(form)
            .build()
            .map_err(Into::into)
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
                let message = body_message_or_default(response, "Unauthorized").await;
                Err(SdkError::Authentication(message))
            }
            StatusCode::FORBIDDEN => {
                let message = body_message_or_default(response, "Forbidden").await;
                Err(SdkError::Authorization(message))
            }
            status if status.is_server_error() => {
                let message = body_message_or_default(response, "Server error").await;
                Err(SdkError::ServerError { status, message })
            }
            status if !status.is_success() => {
                let message = body_message_or_default(response, "Request failed").await;
                Err(SdkError::ServerError { status, message })
            }
            _ => Ok(response),
        }
    }
}

async fn body_message_or_default(response: Response, default: &str) -> String {
    let message = response
        .text()
        .await
        .unwrap_or_else(|_| default.to_string());
    if message.is_empty() {
        default.to_string()
    } else {
        message
    }
}

fn new_default_headers(bearer_token: &str) -> Result<HeaderMap, SdkError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        str_to_header_value(&format!("Bearer {}", bearer_token))?,
    );
    Ok(headers)
}

fn str_to_header_value(value: &str) -> Result<HeaderValue, SdkError> {
    value
        .parse()
        .map_err(|e: InvalidHeaderValue| SdkError::InvalidHeaderValue(e.to_string()))
}

fn new_base_client(headers: &HeaderMap) -> Result<reqwest::Client, SdkError> {
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "Tensorlake Cloud SDK/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .default_headers(headers.clone())
        .build()?;
    Ok(client)
}
