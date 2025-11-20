//! HTTP client that interacts with the Tensorlake Cloud API.
use reqwest::{
    Request, Response, StatusCode,
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware};
use std::{result::Result, sync::Arc};

use crate::error::SdkError;

/// HTTP client that interacts with the Tensorlake Cloud API.
#[derive(Clone)]
pub struct Client {
    base_url: String,
    default_headers: HeaderMap,
    middlewares: Vec<Arc<dyn Middleware + 'static>>,
    client: ClientWithMiddleware,
}

impl Client {
    /// Create a new SDK client without any middleware or scopes.
    pub fn new(base_url: &str, bearer_token: &str) -> Result<Self, SdkError> {
        let default_headers = new_default_headers(bearer_token)?;
        let base_client = new_base_client(&default_headers)?;

        let client = ClientBuilder::new(base_client).build();

        Ok(Self {
            base_url: base_url.to_string(),
            middlewares: Vec::new(),
            default_headers,
            client,
        })
    }

    /// Create a new client with additional middleware.
    ///
    /// This allows you to inject custom middleware such as VCR recording/playback
    /// or other request/response interceptors.
    pub fn with_middleware<M>(self, middleware: M) -> Result<Self, SdkError>
    where
        M: Middleware + 'static,
    {
        let mut middlewares = self.middlewares.clone();
        middlewares.push(Arc::new(middleware));

        let base_client = new_base_client(&self.default_headers)?;
        let mut builder = ClientBuilder::new(base_client);
        for middleware in &self.middlewares {
            builder = builder.with_arc(middleware.clone());
        }

        Ok(Self {
            client: builder.build(),
            middlewares,
            ..self
        })
    }

    /// Create a new client with an organization and project scope.
    ///
    /// This allows you to explicitly set the organization and project IDs for the client.
    pub fn with_scope(self, organization_id: &str, project_id: &str) -> Result<Self, SdkError> {
        let mut default_headers = self.default_headers.clone();
        default_headers.insert(
            "X-Tensorlake-Organization-Id",
            str_to_header_value(organization_id)?,
        );
        default_headers.insert("X-Tensorlake-Project-Id", str_to_header_value(project_id)?);

        let base_client = new_base_client(&default_headers)?;
        let mut builder = ClientBuilder::new(base_client);
        for middleware in &self.middlewares {
            builder = builder.with_arc(middleware.clone());
        }

        Ok(Self {
            client: builder.build(),
            default_headers,
            ..self
        })
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
