//! # Tensorlake Cloud SDK - Secrets
//!
//! This module provides functionality for managing secrets in the Tensorlake Cloud platform.
//!
//! ## Usage
//!
//! ```rust
//! use cloud_sdk::{Sdk, secrets::models::CreateSecret};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key")?;
//!     let secrets_client = sdk.secrets();
//!
//!     // Create a secret
//!     let create_req = CreateSecret {
//!         name: "my-secret".to_string(),
//!         value: "secret-value".to_string(),
//!     };
//!     secrets_client.create("org-id", "project-id", create_req).await?;
//!
//!     // List secrets
//!     secrets_client.list("org-id", "project-id", None, None, None).await?;
//!     Ok(())
//! }
//! ```

pub mod models;

use crate::client::Client;
use miette::IntoDiagnostic;

use models::*;

/// A client for managing secrets in Tensorlake Cloud.
#[derive(Debug)]
pub struct SecretsClient {
    service_url: String,
    client: Client,
}

impl SecretsClient {
    /// Create a new secrets client.
    ///
    /// # Arguments
    ///
    /// * `client` - The base HTTP client configured with authentication
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, secrets::SecretsClient};
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let secrets_client = SecretsClient::new(client);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        Self {
            service_url: format!("{}/platform/v1/organizations", client.base_url()),
            client,
        }
    }

    /// Create a new secret.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The ID of the organization
    /// * `project_id` - The ID of the project
    /// * `create_secret` - The secret creation request
    ///
    /// # Returns
    ///
    /// Returns the created secret.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, secrets::{SecretsClient, models::CreateSecret}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let secrets_client = SecretsClient::new(client);
    ///     let create_req = CreateSecret {
    ///         name: "api-key".to_string(),
    ///         value: "secret123".to_string(),
    ///     };
    ///     secrets_client.create("org-123", "proj-456", create_req).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        organization_id: &str,
        project_id: &str,
        create_secret: CreateSecret,
    ) -> miette::Result<Secret> {
        let uri_str = format!(
            "{}/{organization_id}/projects/{project_id}/secrets",
            self.service_url,
        );

        let req_builder = self
            .client
            .request(reqwest::Method::POST, &uri_str)
            .json(&create_secret);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to create secret");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let secret = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(secret)
    }

    /// Upsert secrets (create or update).
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The ID of the organization
    /// * `project_id` - The ID of the project
    /// * `upsert_secret` - The secret upsert request (single or multiple)
    ///
    /// # Returns
    ///
    /// Returns the upserted secret(s).
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, secrets::{SecretsClient, models::{UpsertSecret, CreateSecret}}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let secrets_client = SecretsClient::new(client);
    ///     let create_req = CreateSecret {
    ///         name: "api-key".to_string(),
    ///         value: "secret123".to_string(),
    ///     };
    ///     let upsert_req = UpsertSecret::Single(create_req);
    ///     secrets_client.upsert("org-123", "proj-456", upsert_req).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn upsert(
        &self,
        organization_id: &str,
        project_id: &str,
        upsert_secret: UpsertSecret,
    ) -> miette::Result<UpsertSecretResponse> {
        let uri_str = format!(
            "{}/{organization_id}/projects/{project_id}/secrets",
            self.service_url
        );

        let req_builder = self
            .client
            .request(reqwest::Method::PUT, &uri_str)
            .json(&upsert_secret);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to upsert secret");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let response = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(response)
    }

    /// List secrets in a project.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The ID of the organization
    /// * `project_id` - The ID of the project
    /// * `next` - Optional cursor for next page
    /// * `prev` - Optional cursor for previous page
    /// * `page_size` - Optional page size (default 10, max 100)
    ///
    /// # Returns
    ///
    /// Returns a list of secrets with pagination information.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, secrets::SecretsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let secrets_client = SecretsClient::new(client);
    ///     secrets_client.list("org-123", "proj-456", None, None, Some(20)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        organization_id: &str,
        project_id: &str,
        next: Option<&str>,
        prev: Option<&str>,
        page_size: Option<i32>,
    ) -> miette::Result<SecretsList> {
        let uri_str = format!(
            "{}/{organization_id}/projects/{project_id}/secrets",
            self.service_url,
        );

        let mut req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        if let Some(ref param_value) = next {
            req_builder = req_builder.query(&[("next", &param_value.to_string())]);
        }
        if let Some(ref param_value) = prev {
            req_builder = req_builder.query(&[("prev", &param_value.to_string())]);
        }
        if let Some(ref param_value) = page_size {
            req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
        }

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to fetch secrets");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let list = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(list)
    }

    /// Get a specific secret by ID.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The ID of the organization
    /// * `project_id` - The ID of the project
    /// * `secret_id` - The ID of the secret
    ///
    /// # Returns
    ///
    /// Returns the secret details.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, secrets::SecretsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let secrets_client = SecretsClient::new(client);
    ///     secrets_client.get("org-123", "proj-456", "secret-789").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &self,
        organization_id: &str,
        project_id: &str,
        secret_id: &str,
    ) -> miette::Result<Secret> {
        let uri_str = format!(
            "{}/platform/v1/organizations/{}/projects/{}/secrets/{}",
            self.client.base_url(),
            organization_id,
            project_id,
            secret_id
        );

        let req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to retrieve secret");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let secret = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(secret)
    }

    /// Delete a secret.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The ID of the organization
    /// * `project_id` - The ID of the project
    /// * `secret_id` - The ID of the secret to delete
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, secrets::SecretsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let secrets_client = SecretsClient::new(client);
    ///     secrets_client.delete("org-123", "proj-456", "secret-789").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(
        &self,
        organization_id: &str,
        project_id: &str,
        secret_id: &str,
    ) -> miette::Result<()> {
        let uri_str = format!(
            "{}/platform/v1/organizations/{}/projects/{}/secrets/{}",
            self.client.base_url(),
            organization_id,
            project_id,
            secret_id
        );

        let req_builder = self.client.request(reqwest::Method::DELETE, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if !resp.status().is_success() {
            miette::bail!("Unable to delete secret");
        }

        Ok(())
    }
}
