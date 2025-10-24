//! # Tensorlake Cloud SDK
//!
//! A comprehensive Rust SDK for interacting with Tensorlake Cloud APIs.
//! This SDK provides a high-level, ergonomic interface for managing applications,
//! functions, and execution requests in the Tensorlake Cloud platform.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use cloud_sdk::Sdk;
//!
//! fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create the SDK client
//! let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key").unwrap();
//!
//! // Get the applications client
//! let apps_client = sdk.applications();
//!
//! // List applications in the default namespace
//! // let apps = apps_client.list("default", None, None, None).await?;
//! // println!("Found {} applications", apps.applications.len());
//!
//! Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! The SDK uses Bearer token authentication, either a Personal Access Token (PAT) or a Project API key.
//! Provide your token when creating the SDK:
//!
//! ```rust,no_run
//! use cloud_sdk::Sdk;
//!
//! let sdk = Sdk::new("https://api.tensorlake.ai", "your-token").unwrap();
//! ```
//!
//! ## Available Clients
//!
//! - [`ApplicationsClient`](applications::ApplicationsClient): Manage applications, functions, and requests
//!
//! ## Error Handling
//!
//! The SDK provides detailed error types for different scenarios:
//!
//! ```rust,no_run
//! use cloud_sdk::{Sdk, applications::apis};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key").unwrap();
//! let apps_client = sdk.applications();
//!
//! match apps_client.list("default", None, None, None).await {
//!     Ok(apps) => println!("Success: {:?}", apps.applications.len()),
//!     Err(apis::Error::Reqwest(e)) => eprintln!("Network error: {}", e),
//!     Err(apis::Error::Serde(e)) => eprintln!("Serialization error: {}", e),
//!     Err(apis::Error::ResponseError(content)) => {
//!         eprintln!("API error {}: {}", content.status, content.content)
//!     }
//!     _ => eprintln!("Other error"),
//! }
//! Ok(())
//! }
//! ```

pub mod applications;
pub use applications::*;

mod client;
pub use client::*;

/// The main entry point for the Tensorlake Cloud SDK.
///
/// The `Sdk` struct provides a unified interface to all Tensorlake Cloud services.
/// It manages authentication and provides access to various service clients.
///
/// ## Example
///
/// ```rust,no_run
/// use cloud_sdk::Sdk;
///
/// fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key").unwrap();
///
/// // Access different service clients
/// let apps_client = sdk.applications();
/// Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Sdk {
    client: Client,
}

impl Sdk {
    /// Create a new SDK instance with the specified base URL and bearer token.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Tensorlake Cloud API (e.g., "https://api.tensorlake.ai")
    /// * `bearer_token` - Your API key for authentication
    ///
    /// # Returns
    ///
    /// Returns a new `Sdk` instance configured with the provided credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created or configured.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cloud_sdk::Sdk;
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key").unwrap();
    /// Ok(())
    /// }
    /// ```
    pub fn new(base_url: &str, bearer_token: &str) -> miette::Result<Self> {
        let client = Client::new(base_url, bearer_token)?;
        Ok(Self { client })
    }

    /// Get a client for managing applications and requests.
    ///
    /// This method returns an [`ApplicationsClient`] that provides methods for:
    /// - Listing, creating, updating, and deleting applications
    /// - Invoking applications with data
    /// - Managing execution requests
    ///
    /// # Returns
    ///
    /// Returns an [`ApplicationsClient`] instance configured with the SDK's authentication.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cloud_sdk::Sdk;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key").unwrap();
    /// let apps_client = sdk.applications();
    ///
    /// // Use the applications client
    /// let apps = apps_client.list("default", None, None, None).await?;
    /// Ok(())
    /// }
    /// ```
    pub fn applications(&self) -> ApplicationsClient {
        ApplicationsClient::new(self.client.clone())
    }
}
