//! # Tensorlake Cloud SDK - Applications
//!
//! This module provides a high-level, ergonomic interface for interacting with Tensorlake Cloud applications.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::{ListApplicationsRequest, GetApplicationRequest}}};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
//! let apps_client = ApplicationsClient::new(client);
//!
//! // List applications in a namespace
//! let request = ListApplicationsRequest::builder()
//!     .namespace("default".to_string())
//!     .build()?;
//! let apps = apps_client.list(&request).await?;
//!
//! // Get a specific application
//! let app = apps_client.get(&GetApplicationRequest::builder()
//!     .namespace("default".to_string())
//!     .application("my-app".to_string())
//!     .build()?).await?;
//!
//! Ok(())
//! }
//! ```

pub mod error;
pub mod models;

use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest::{
    Method, StatusCode,
    header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE},
    multipart::{Form, Part},
};
use std::io::Error;
use tokio_util::{codec::FramedRead, io::StreamReader};

use crate::{client::Client, error::SdkError, event_source::SseDecoder};

/// Response from invoking an application
pub enum InvokeResponse {
    /// The request ID of the invocation
    RequestId(String),
    /// A stream of progress events
    Stream(
        std::pin::Pin<
            Box<
                dyn futures::Stream<Item = Result<models::RequestStateChangeEvent, SdkError>>
                    + Send,
            >,
        >,
    ),
}

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
    /// use tensorlake_cloud_sdk::{Client, applications::ApplicationsClient};
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
    /// * `request` - The list applications request
    ///
    /// # Returns
    ///
    /// Returns a list of applications in the specified namespace.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::ApplicationsClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = tensorlake_cloud_sdk::applications::models::ListApplicationsRequest {
    ///         namespace: "default".to_string(),
    ///         limit: Some(10),
    ///         cursor: None,
    ///         direction: None,
    ///     };
    ///     apps_client.list(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &models::ListApplicationsRequest,
    ) -> Result<models::ApplicationsList, SdkError> {
        let uri_str = format!("/v1/namespaces/{}/applications", request.namespace);
        let mut req_builder = self.client.request(Method::GET, &uri_str);

        if let Some(ref param_value) = request.limit {
            req_builder = req_builder.query(&[("limit", param_value)]);
        }
        if let Some(ref param_value) = request.cursor {
            req_builder = req_builder.query(&[("cursor", param_value)]);
        }
        if let Some(ref param_value) = request.direction {
            req_builder = req_builder.query(&[("direction", param_value)]);
        }

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let bytes = resp.bytes().await?;
        let jd = &mut serde_json::Deserializer::from_slice(bytes.as_ref());
        let list = serde_path_to_error::deserialize(jd)?;

        Ok(list)
    }

    /// Get details of a specific application.
    ///
    /// # Arguments
    ///
    /// * `request` - The get application request
    ///
    /// # Returns
    ///
    /// Returns the application details.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::GetApplicationRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = GetApplicationRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .build()?;
    ///     apps_client.get(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &models::GetApplicationRequest,
    ) -> Result<models::Application, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}",
            request.namespace, request.application
        );
        let req_builder = self.client.request(Method::GET, &uri_str);

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let bytes = resp.bytes().await?;
        let jd = &mut serde_json::Deserializer::from_reader(bytes.as_ref());
        let app = serde_path_to_error::deserialize(jd)?;

        Ok(app)
    }

    /// Create or update an application.
    ///
    /// # Arguments
    ///
    /// * `request` - The upsert application request
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::{UpsertApplicationRequest, ApplicationManifest}}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     // Note: Requires constructing a full ApplicationManifest model with all required fields
    ///     // This is typically done by parsing from configuration files or build manifests
    ///     let code_zip: Vec<u8> = vec![/* zip file bytes */];
    ///     let app_data = ApplicationManifest::builder()
    ///         .name("my-app")
    ///         .version("1.0.0")
    ///         .build()?;
    ///     let request = UpsertApplicationRequest::builder()
    ///         .namespace("default")
    ///         .application_manifest(app_data)
    ///         .code_zip(code_zip)
    ///         .build()?;
    ///     apps_client.upsert(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn upsert(&self, request: &models::UpsertApplicationRequest) -> Result<(), SdkError> {
        let mut multipart_form = Form::new();

        let manifest_json = serde_json::to_string(&request.application_manifest)?;
        multipart_form = multipart_form.text("application", manifest_json);

        let file_part = Part::bytes(request.code_zip.clone()).file_name("code.zip");
        multipart_form = multipart_form.part("code", file_part);

        let uri_str = format!("/v1/namespaces/{}/applications", request.namespace);
        let req = self
            .client
            .build_multipart_request(Method::POST, &uri_str, multipart_form)?;
        let _resp = self.client.execute(req).await?;

        Ok(())
    }

    /// Delete an application.
    ///
    /// # Arguments
    ///
    /// * `request` - The delete application request
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::DeleteApplicationRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = DeleteApplicationRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .build()?;
    ///     apps_client.delete(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(&self, request: &models::DeleteApplicationRequest) -> Result<(), SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}",
            request.namespace, request.application
        );
        let req_builder = self.client.request(Method::DELETE, &uri_str);

        let req = req_builder.build()?;
        let _resp = self.client.execute(req).await?;

        Ok(())
    }

    /// Invoke an application with object data.
    ///
    /// # Arguments
    ///
    /// * `request` - The invoke application request
    ///
    /// # Returns
    ///
    /// If `stream` is false, returns the request ID. If `stream` is true, returns a stream of progress events.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, InvokeResponse, models::InvokeApplicationRequest}};
    /// use serde_json;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let data = serde_json::json!({"input": "hello world"});
    ///     let request = InvokeApplicationRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .body(data)
    ///         .build()?;
    ///     let response = apps_client.invoke(&request).await?;
    ///     match response {
    ///         InvokeResponse::RequestId(id) => println!("Request ID: {}", id),
    ///         InvokeResponse::Stream(_) => unreachable!(),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn invoke(
        &self,
        request: &models::InvokeApplicationRequest,
    ) -> Result<InvokeResponse, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}",
            request.namespace, request.application
        );
        let req_builder = self.client.base_request(Method::POST, &uri_str);

        let req = if request.stream {
            req_builder
                .header(ACCEPT, "text/event-stream")
                .json(&request.body)
                .build()?
        } else {
            req_builder
                .header(ACCEPT, "application/json")
                .json(&request.body)
                .build()?
        };
        let resp = self.client.execute(req).await?;

        if request.stream {
            let decoder: SseDecoder<models::RequestStateChangeEvent> = SseDecoder::new();
            let stream = resp.bytes_stream();
            let frame = FramedRead::new(StreamReader::new(stream.map_err(Error::other)), decoder);
            Ok(InvokeResponse::Stream(Box::pin(frame.into_stream())))
        } else {
            let bytes = resp.bytes().await?;
            let jd = &mut serde_json::Deserializer::from_slice(&bytes);
            let request_id_resp: serde_json::Value = serde_path_to_error::deserialize(jd)?;
            let request_id =
                request_id_resp["request_id"]
                    .as_str()
                    .ok_or_else(|| SdkError::ServerError {
                        status: reqwest::StatusCode::OK,
                        message: "Missing request_id in response".to_string(),
                    })?;
            Ok(InvokeResponse::RequestId(request_id.to_string()))
        }
    }

    /// List requests for an application.
    ///
    /// # Arguments
    ///
    /// * `request` - The list requests request
    ///
    /// # Returns
    ///
    /// Returns the list of requests for the application.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::ListRequestsRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = ListRequestsRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .limit(10)
    ///         .build()?;
    ///     apps_client.list_requests(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_requests(
        &self,
        request: &models::ListRequestsRequest,
    ) -> Result<models::ApplicationRequests, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}/requests",
            request.namespace, request.application
        );
        let mut req_builder = self.client.request(Method::GET, &uri_str);

        if let Some(ref param_value) = request.limit {
            req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
        }
        if let Some(ref param_value) = request.cursor {
            req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
        }
        if let Some(ref param_value) = request.direction {
            req_builder = req_builder.query(&[("direction", &param_value.to_string())]);
        }

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let bytes = resp.bytes().await?;
        let jd = &mut serde_json::Deserializer::from_reader(bytes.as_ref());
        let list = serde_path_to_error::deserialize(jd)?;

        Ok(list)
    }

    /// Delete a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The delete request request
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::DeleteRequestRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = DeleteRequestRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .request_id("request-123")
    ///         .build()?;
    ///     apps_client.delete_request(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_request(
        &self,
        request: &models::DeleteRequestRequest,
    ) -> Result<(), SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}/requests/{}",
            request.namespace, request.application, request.request_id
        );
        let req_builder = self.client.request(Method::DELETE, &uri_str);

        let req = req_builder.build()?;
        let _resp = self.client.execute(req).await?;

        Ok(())
    }

    /// Download the output of a specific function call within a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The download function output request
    ///
    /// # Returns
    ///
    /// Returns the function call output data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::DownloadFunctionOutputRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = DownloadFunctionOutputRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .request_id("request-123")
    ///         .function_call_id("func-456")
    ///         .build()?;
    ///     apps_client.download_function_output(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_function_output(
        &self,
        request: &models::DownloadFunctionOutputRequest,
    ) -> Result<models::DownloadOutput, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}/requests/{}/output/{}",
            request.namespace, request.application, request.request_id, request.function_call_id
        );
        let req_builder = self.client.request(reqwest::Method::GET, &uri_str);

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let mut output = models::DownloadOutput {
            content_type: resp.headers().get(CONTENT_TYPE).cloned(),
            content_length: resp.headers().get(CONTENT_LENGTH).cloned(),
            content: Bytes::new(),
        };

        if resp.status().is_success() {
            output.content = resp.bytes().await?;
        }

        Ok(output)
    }

    /// Stream progress events for a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The stream progress request
    ///
    /// # Returns
    ///
    /// A stream that yields progress events for the request.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::StreamProgressRequest}};
    /// use futures::StreamExt;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = StreamProgressRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .request_id("request-123")
    ///         .build()?;
    ///     let mut stream = apps_client.stream_progress(&request).await?;
    ///     while let Some(event) = stream.next().await {
    ///         match event {
    ///             Ok(event) => println!("Event: {:?}", event),
    ///             Err(e) => eprintln!("Error: {:?}", e),
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn stream_progress(
        &self,
        request: &models::StreamProgressRequest,
    ) -> Result<impl Stream<Item = Result<models::RequestStateChangeEvent, SdkError>>, SdkError>
    {
        let uri_str = format!(
            "/namespaces/{}/applications/{}/requests/{}/progress",
            request.namespace, request.application, request.request_id
        );
        let req_builder = self
            .client
            .request(Method::GET, &uri_str)
            .header(ACCEPT, "text/event-stream");

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let decoder: SseDecoder<models::RequestStateChangeEvent> = SseDecoder::new();
        let stream = resp.bytes_stream();
        let frame = FramedRead::new(StreamReader::new(stream.map_err(Error::other)), decoder);

        Ok(frame.into_stream())
    }

    /// Check if output is available for a request without downloading the content.
    ///
    /// This performs a HEAD request to check for the presence of output data.
    ///
    /// # Arguments
    ///
    /// * `request` - The check function output request
    ///
    /// # Returns
    ///
    /// Returns `Some` with output metadata if available, or `None` if no output exists.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::CheckFunctionOutputRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = CheckFunctionOutputRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .request_id("request-123")
    ///         .build()?;
    ///     if let Some(metadata) = apps_client.check_function_output(&request).await? {
    ///         println!("Output available, size: {:?}", metadata.content_length);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn check_function_output(
        &self,
        request: &models::CheckFunctionOutputRequest,
    ) -> Result<Option<models::DownloadOutput>, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}/requests/{}/output",
            request.namespace, request.application, request.request_id
        );
        let req_builder = self.client.request(Method::HEAD, &uri_str);

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

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
    /// * `request` - The download request output request
    ///
    /// # Returns
    ///
    /// Returns the complete request output data.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::applications::{ApplicationsClient, models::DownloadRequestOutputRequest};
    ///
    /// async fn example(apps_client: &ApplicationsClient) -> Result<(), Box<dyn std::error::Error>> {
    ///     let request = DownloadRequestOutputRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .request_id("request-123")
    ///         .build()?;
    ///     let output = apps_client.download_request_output(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn download_request_output(
        &self,
        request: &models::DownloadRequestOutputRequest,
    ) -> Result<models::DownloadOutput, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}/requests/{}/output",
            request.namespace, request.application, request.request_id
        );
        let req_builder = self.client.request(Method::GET, &uri_str);

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let mut output = models::DownloadOutput {
            content_type: resp.headers().get(CONTENT_TYPE).cloned(),
            content_length: resp.headers().get(CONTENT_LENGTH).cloned(),
            content: Bytes::new(),
        };

        if resp.status().is_success() {
            output.content = resp.bytes().await?;
        }

        Ok(output)
    }

    /// Get logs for an application.
    ///
    /// # Arguments
    ///
    /// * `request` - The get logs request
    ///
    /// # Returns
    ///
    /// Returns the logs for the application matching the request filters.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tensorlake_cloud_sdk::{Client, applications::{ApplicationsClient, models::GetLogsRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let apps_client = ApplicationsClient::new(client);
    ///     let request = GetLogsRequest::builder()
    ///         .namespace("default")
    ///         .application("my-app")
    ///         .tail(100)
    ///         .build()?;
    ///     apps_client.get_logs(&request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_logs(
        &self,
        request: &models::GetLogsRequest,
    ) -> Result<models::EventsResponse, SdkError> {
        let uri_str = format!(
            "/v1/namespaces/{}/applications/{}/logs",
            request.namespace, request.application
        );
        let mut req_builder = self.client.request(Method::GET, &uri_str);

        if let Some(ref param_value) = request.request_id {
            req_builder = req_builder.query(&[("requestId", param_value)]);
        }
        if let Some(ref param_value) = request.container_id {
            req_builder = req_builder.query(&[("containerId", param_value)]);
        }
        if let Some(ref param_value) = request.function {
            req_builder = req_builder.query(&[("function", param_value)]);
        }
        if let Some(ref param_value) = request.next_token {
            req_builder = req_builder.query(&[("nextToken", param_value)]);
        }
        if let Some(param_value) = request.head {
            req_builder = req_builder.query(&[("head", &param_value.to_string())]);
        }
        if let Some(param_value) = request.tail {
            req_builder = req_builder.query(&[("tail", &param_value.to_string())]);
        }
        if let Some(ref param_value) = request.ignore {
            req_builder = req_builder.query(&[("ignore", param_value)]);
        }
        if let Some(ref param_value) = request.function_executor {
            req_builder = req_builder.query(&[("functionExecutor", param_value)]);
        }

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let bytes = resp.bytes().await?;
        let jd = &mut serde_json::Deserializer::from_reader(bytes.as_ref());
        let events_resp = serde_path_to_error::deserialize(jd)?;

        Ok(events_resp)
    }
}
