//! # Tensorlake Cloud SDK - Applications
//!
//! This module provides a high-level, ergonomic interface for interacting with Tensorlake Cloud applications.
//!
//! ## Usage
//!
//! ```rust
//! use cloud_sdk::{Client, applications::ApplicationsClient};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
//! let apps_client = ApplicationsClient::new(client);
//!
//! // List applications in a namespace
//! let apps = apps_client.list("default", None, None, None).await?;
//!
//! // Get a specific application
//! let app = apps_client.get("default", "my-app").await?;
//!
//! // Invoke an application
//! let data = serde_json::json!({"input": "hello"});
//! apps_client.send_request("default", "my-app", data).await?;
//! Ok(())
//! }
//! ```

pub mod models;

use bytes::Bytes;
use miette::IntoDiagnostic;
use reqwest::{
    Method, StatusCode,
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    multipart::{Form, Part},
};

use crate::client::Client;

/// A client for interacting with Tensorlake Cloud applications.
///
/// This client provides high-level methods for managing applications, requests, and related operations.
/// It wraps the raw API calls with a more ergonomic interface.
#[derive(Debug)]
pub struct ApplicationsClient {
    client: Client,
}

impl ApplicationsClient {
    /// Create a new applications client.
    ///
    /// # Arguments
    ///
    /// * `client` - The base HTTP client configured with authentication
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    /// let apps_client = ApplicationsClient::new(client);
    /// Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// List all applications in a namespace.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace to list applications from
    /// * `limit` - Optional limit on number of applications to return
    /// * `cursor` - Optional cursor for pagination
    /// * `direction` - Optional direction for cursor-based pagination
    ///
    /// # Returns
    ///
    /// Returns a list of applications in the specified namespace.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     apps_client.list("default", Some(10), None, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        namespace: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
        direction: Option<models::CursorDirection>,
    ) -> miette::Result<models::ApplicationsList> {
        let uri_str = format!("/v1/namespaces/{namespace}/applications");
        let mut req_builder = self.client.request(Method::GET, &uri_str);

        if let Some(ref param_value) = limit {
            req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = cursor {
            req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
        }
        if let Some(ref param_value) = direction {
            req_builder = req_builder.query(&[("direction", &param_value.to_string())]);
        }

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to fetch applications");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let list = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(list)
    }

    /// Get details of a specific application.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application
    ///
    /// # Returns
    ///
    /// Returns the application details.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     apps_client.get("default", "my-app").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &self,
        namespace: &str,
        application: &str,
    ) -> miette::Result<models::Application> {
        let uri_str = format!("/v1/namespaces/{namespace}/applications/{application}",);
        let req_builder = self.client.request(Method::GET, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to retrieve application");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let app = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(app)
    }

    /// Create or update an application.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace for the application
    /// * `application` - The application model data
    /// * `code_zip` - The application code as ZIP file data
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     // Note: Requires constructing a full Application model with all required fields
    ///     // This is typically done by parsing from configuration files or build manifests
    ///     let code_zip: Vec<u8> = vec![/* zip file bytes */];
    ///     // let app_data = Application { ... }; // construct full application data
    ///     // apps_client.upsert("default", app_data, code_zip).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn upsert(
        &self,
        namespace: &str,
        application: models::Application,
        code_zip: Vec<u8>,
    ) -> miette::Result<()> {
        let mut multipart_form = Form::new();

        let manifest_json = serde_json::to_string(&application).into_diagnostic()?;
        multipart_form = multipart_form.text("application", manifest_json);

        let file_part = Part::bytes(code_zip).file_name("code.zip");
        multipart_form = multipart_form.part("code", file_part);

        let uri_str = format!("/v1/namespaces/{namespace}/applications");
        let req_builder = self
            .client
            .request(Method::POST, &uri_str)
            .multipart(multipart_form);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to upsert application");
        }

        Ok(())
    }

    /// Delete an application.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application to delete
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     apps_client.delete("default", "my-app").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, namespace: &str, application: &str) -> miette::Result<()> {
        let uri_str = format!("/v1/namespaces/{namespace}/applications/{application}");
        let req_builder = self.client.request(Method::DELETE, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if !resp.status().is_success() {
            miette::bail!("Unable to delete application");
        }

        Ok(())
    }

    /// Invoke an application with object data.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application to invoke
    /// * `body` - JSON data to send with the invocation
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    /// use serde_json;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let data = serde_json::json!({"input": "hello world"});
    ///     apps_client.send_request("default", "my-app", data).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_request(
        &self,
        namespace: &str,
        application: &str,
        body: serde_json::Value,
    ) -> miette::Result<()> {
        let uri_str = format!("/v1/namespaces/{namespace}/applications/{application}");
        let mut req_builder = self.client.request(Method::POST, &uri_str);
        req_builder = req_builder.json(&body);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if !resp.status().is_success() {
            miette::bail!("Unable to invoke application");
        }

        Ok(())
    }

    /// List requests for an application.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application
    /// * `limit` - Optional limit on number of requests to return
    /// * `cursor` - Optional cursor for pagination
    /// * `direction` - Optional direction for cursor-based pagination
    ///
    /// # Returns
    ///
    /// Returns the list of requests for the application.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     apps_client.list_requests("default", "my-app", Some(10), None, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_requests(
        &self,
        namespace: &str,
        application: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
        direction: Option<models::CursorDirection>,
    ) -> miette::Result<models::ApplicationRequests> {
        let uri_str = format!("/v1/namespaces/{namespace}/applications/{application}/requests");
        let mut req_builder = self.client.request(Method::GET, &uri_str);

        if let Some(ref param_value) = limit {
            req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = cursor {
            req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
        }
        if let Some(ref param_value) = direction {
            req_builder = req_builder.query(&[("direction", &param_value.to_string())]);
        }

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to fetch application requests");
        }

        let bytes = resp.bytes().await.into_diagnostic()?;
        let list = serde_json::from_reader(bytes.as_ref()).into_diagnostic()?;

        Ok(list)
    }

    /// Delete a request.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application
    /// * `request_id` - The ID of the request to delete
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     apps_client.delete_request("default", "my-app", "request-123").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_request(
        &self,
        namespace: &str,
        application: &str,
        request_id: &str,
    ) -> miette::Result<()> {
        let uri_str =
            format!("/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}");
        let req_builder = self.client.request(Method::DELETE, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        let status = resp.status();

        if !status.is_success() {
            miette::bail!("Unable to delete request");
        }
        Ok(())
    }

    /// Download the output of a specific function call within a request.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application
    /// * `request_id` - The ID of the request
    /// * `function_call_id` - The ID of the specific function call
    ///
    /// # Returns
    ///
    /// Returns the function call output data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     apps_client.download_function_output("default", "my-app", "request-123", "func-456").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_function_output(
        &self,
        namespace: &str,
        application: &str,
        request_id: &str,
        function_call_id: &str,
    ) -> miette::Result<models::DownloadOutput> {
        let uri_str = format!(
            "/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}/output/{function_call_id}"
        );
        let req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to download function output");
        }

        let mut output = models::DownloadOutput {
            content_type: resp.headers().get(CONTENT_TYPE).cloned(),
            content_length: resp.headers().get(CONTENT_LENGTH).cloned(),
            content: Bytes::new(),
        };

        if resp.status().is_success() {
            output.content = resp.bytes().await.into_diagnostic()?;
        }

        Ok(output)
    }

    /// Check if output is available for a request without downloading the content.
    ///
    /// This performs a HEAD request to check for the presence of output data.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application
    /// * `request_id` - The ID of the request
    ///
    /// # Returns
    ///
    /// Returns `Some` with output metadata if available, or `None` if no output exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     if let Some(metadata) = apps_client.check_function_output("default", "my-app", "request-123").await? {
    ///         println!("Output available, size: {:?}", metadata.content_length);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn check_function_output(
        &self,
        namespace: &str,
        application: &str,
        request_id: &str,
    ) -> miette::Result<Option<models::DownloadOutput>> {
        let uri_str = format!(
            "/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}/output"
        );
        let req_builder = self.client.request(Method::HEAD, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to check function output");
        }

        if resp.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        Ok(Some(models::DownloadOutput {
            content_type: resp.headers().get(CONTENT_TYPE).cloned(),
            content_length: resp.headers().get(CONTENT_LENGTH).cloned(),
            content: Bytes::new(),
        }))
    }

    /// Download the complete output of a request.
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace containing the application
    /// * `application` - The name of the application
    /// * `request_id` - The ID of the request
    ///
    /// # Returns
    ///
    /// Returns the complete request output data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use cloud_sdk::applications::ApplicationsClient;
    ///
    /// async fn example(apps_client: &ApplicationsClient) -> Result<(), Box<dyn std::error::Error>> {
    ///     let output = apps_client.download_request_output("default", "my-app", "request-123").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_request_output(
        &self,
        namespace: &str,
        application: &str,
        request_id: &str,
    ) -> miette::Result<models::DownloadOutput> {
        let uri_str = format!(
            "/v1/namespaces/{namespace}/applications/{application}/requests/{request_id}/output",
        );
        let req_builder = self.client.request(Method::GET, &uri_str);

        let req = req_builder.build().into_diagnostic()?;
        let resp = self.client.execute(req).await.into_diagnostic()?;

        if resp.status().is_server_error() {
            miette::bail!("Unable to download request output");
        }

        let mut output = models::DownloadOutput {
            content_type: resp.headers().get(CONTENT_TYPE).cloned(),
            content_length: resp.headers().get(CONTENT_LENGTH).cloned(),
            content: Bytes::new(),
        };

        if resp.status().is_success() {
            output.content = resp.bytes().await.into_diagnostic()?;
        }

        Ok(output)
    }
}
