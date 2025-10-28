//! HTTP client that interacts with the Tensorlake Cloud API.
use miette::{Context, IntoDiagnostic};
use reqwest::{Request, Response, header::HeaderMap};

/// HTTP client that interacts with the Tensorlake Cloud API.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    /// Create a new SDK client.
    pub fn new(base_url: &str, bearer_token: &str) -> miette::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", bearer_token)
                .parse()
                .into_diagnostic()?,
        );

        let client = reqwest::Client::builder()
            .user_agent(format!(
                "Tensorlake Cloud SDK/{}",
                env!("CARGO_PKG_VERSION")
            ))
            .default_headers(headers)
            .build()
            .into_diagnostic()
            .wrap_err("Failed to build SDK client")?;

        Ok(Self {
            base_url: base_url.to_string(),
            client,
        })
    }

    /// Execute an HTTP request.
    pub async fn execute(&self, request: Request) -> reqwest::Result<Response> {
        self.client.execute(request).await
    }

    pub fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client.request(method, self.base_url.clone() + path)
    }
}
