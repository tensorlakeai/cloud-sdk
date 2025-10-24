//! HTTP client that interacts with the Tensorlake Cloud API.
use miette::{Context, IntoDiagnostic};
use reqwest::header::HeaderMap;

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

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the HTTP client.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
}
