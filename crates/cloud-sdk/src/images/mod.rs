//! # Tensorlake Cloud SDK - Images
//!
//! This module provides functionality for building and managing container images
//! in the Tensorlake Cloud platform.
//!
//! ## Usage
//!
//! ```rust
//! use cloud_sdk::{Sdk, images::models::ImageBuildRequest};
//!
//! let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key").unwrap();
//! let images_client = sdk.images();
//!
//! // Build an image
//! let build_request = ImageBuildRequest {
//!     image_name: "my-app".to_string(),
//!     image_tag: "latest".to_string(),
//!     context_data: vec![/* tar.gz context data */],
//!     application_name: "my-app".to_string(),
//!     application_version: "1.0.0".to_string(),
//!     function_name: "main".to_string(),
//! };
//!
//! images_client.build_image(build_request);
//! ```

use std::io::Error;

use crate::{client::Client, error::SdkError, event_source::SseDecoder};
use futures::{TryStreamExt, stream::Stream};
use hex;
use reqwest::{Method, header::ACCEPT, multipart::Form};
use sha2::{Digest, Sha256};
use tokio_util::{codec::FramedRead, io::StreamReader};

pub mod error;
pub mod models;
use models::*;

/// A client for managing image builds in Tensorlake Cloud.
#[derive(Debug)]
pub struct ImagesClient {
    client: Client,
}

impl ImagesClient {
    /// Create a new images client.
    ///
    /// # Arguments
    ///
    /// * `client` - The base HTTP client configured with authentication
    /// * `build_service_url` - The URL of the image build service
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, images::ImagesClient};
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Build a container image.
    ///
    /// This method submits an image build request to the Tensorlake Cloud build service
    /// and polls for completion.
    ///
    /// # Arguments
    ///
    /// * `request` - The image build request containing all necessary parameters
    ///
    /// # Returns
    ///
    /// Returns the build result containing the build ID and final status.
    ///
    /// # Errors
    ///
    /// Returns an error if the build request fails or the build process encounters an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, images::{ImagesClient, models::ImageBuildRequest}};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     let request = ImageBuildRequest {
    ///         image_name: "my-image".to_string(),
    ///         image_tag: "v1.0".to_string(),
    ///         context_data: vec![/* context tar.gz data */],
    ///         application_name: "my-app".to_string(),
    ///         application_version: "1.0.0".to_string(),
    ///         function_name: "main".to_string(),
    ///     };
    ///
    ///     images_client.build_image(request).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn build_image(
        &self,
        request: ImageBuildRequest,
    ) -> Result<ImageBuildResult, SdkError> {
        let build_info = self.submit_build_request(&request).await?;
        self.poll_build_status(&build_info.id).await
    }

    /// Submit a build request to the build service.
    async fn submit_build_request(
        &self,
        request: &ImageBuildRequest,
    ) -> Result<BuildInfo, SdkError> {
        let form = Form::new()
            .text("graph_name", request.application_name.clone())
            .text("graph_version", request.application_version.clone())
            .text("graph_function_name", request.function_name.clone())
            .text("image_hash", self.calculate_image_hash(request))
            .text("image_name", request.image_name.clone())
            .part(
                "context",
                reqwest::multipart::Part::bytes(request.context_data.clone())
                    .file_name("context.tar.gz"),
            );

        let request = self
            .client
            .request(Method::PUT, "/images/v2/builds")
            .multipart(form)
            .build()?;

        let response = self.client.execute(request).await?;
        let json = response.json::<BuildInfo>().await?;

        Ok(json)
    }

    /// Poll the build status until completion.
    async fn poll_build_status(&self, build_id: &str) -> Result<ImageBuildResult, SdkError> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 10 {
                return Err(error::ImagesError::BuildTimeout { attempts }.into());
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            let uri_str = format!("/images/v2/builds/{build_id}");
            let request = self.client.request(Method::GET, &uri_str).build()?;

            let response = self.client.execute(request).await?;

            let build_info: BuildInfo = response.json().await?;

            match build_info.status.as_str() {
                "completed" | "succeeded" => {
                    return Ok(ImageBuildResult {
                        id: build_info.id,
                        status: BuildStatus::Succeeded,
                        created_at: build_info.created_at,
                        finished_at: build_info.finished_at,
                        error_message: None,
                    });
                }
                "failed" => {
                    return Ok(ImageBuildResult {
                        id: build_info.id,
                        status: BuildStatus::Failed,
                        created_at: build_info.created_at,
                        finished_at: build_info.finished_at,
                        error_message: build_info.error_message,
                    });
                }
                _ => {
                    // Continue polling for other statuses (pending, in_progress, building, etc.)
                    continue;
                }
            }
        }
    }

    /// List builds for the current project.
    ///
    /// # Arguments
    ///
    /// * `page` - Optional page number (default: 1)
    /// * `page_size` - Optional page size (default: 25, max: 100)
    /// * `status` - Optional filter by build status
    /// * `application_name` - Optional filter by application name
    /// * `image_name` - Optional filter by image name
    /// * `function_name` - Optional filter by function name
    ///
    /// # Returns
    ///
    /// Returns a paginated list of builds.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, images::ImagesClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     images_client.list_builds(Some(1), Some(25), None, None, None, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_builds(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        status: Option<&BuildStatus>,
        application_name: Option<&str>,
        image_name: Option<&str>,
        function_name: Option<&str>,
    ) -> Result<Page<BuildListResponse>, SdkError> {
        let mut query_params = Vec::new();
        if let Some(p) = page {
            query_params.push(("page", p.to_string()));
        }
        if let Some(ps) = page_size {
            query_params.push(("page_size", ps.to_string()));
        }
        if let Some(s) = status {
            // Assuming BuildStatus can be converted to string
            let status_str = match s {
                BuildStatus::Pending => "pending",
                BuildStatus::Enqueued => "enqueued",
                BuildStatus::Building => "building",
                BuildStatus::Succeeded => "succeeded",
                BuildStatus::Failed => "failed",
                BuildStatus::Canceling => "canceling",
                BuildStatus::Canceled => "canceled",
            };
            query_params.push(("status", status_str.to_string()));
        }
        if let Some(gn) = application_name {
            query_params.push(("graph_name", gn.to_string()));
        }
        if let Some(iname) = image_name {
            query_params.push(("image_name", iname.to_string()));
        }
        if let Some(gfn) = function_name {
            query_params.push(("graph_function_name", gfn.to_string()));
        }

        let request = self
            .client
            .request(Method::GET, "/images/v2/builds")
            .query(&query_params)
            .build()?;

        let response = self.client.execute(request).await?;

        Ok(response.json::<Page<BuildListResponse>>().await?)
    }

    /// Cancel a build.
    ///
    /// # Arguments
    ///
    /// * `build_id` - The ID of the build to cancel
    ///
    /// # Returns
    ///
    /// Returns a success message if the cancel request was accepted.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, images::ImagesClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     images_client.cancel_build("build-123").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn cancel_build(&self, build_id: &str) -> Result<(), SdkError> {
        let uri_str = format!("/images/v2/builds/{build_id}/cancel");
        let request = self.client.request(Method::POST, &uri_str).build()?;

        let _response = self.client.execute(request).await?;

        // 202 Accepted, no body
        Ok(())
    }

    /// Get build info.
    ///
    /// # Arguments
    ///
    /// * `build_id` - The ID of the build to get info for
    ///
    /// # Returns
    ///
    /// Returns the build info response containing details about the build.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, images::ImagesClient};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     images_client.get_build_info("build-123").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_build_info(&self, build_id: &str) -> Result<BuildInfoResponse, SdkError> {
        let uri_str = format!("/images/v2/builds/{build_id}");
        let request = self.client.request(Method::GET, &uri_str).build()?;

        let response = self.client.execute(request).await?;

        Ok(response.json::<BuildInfoResponse>().await?)
    }

    /// Stream build logs.
    ///
    /// # Arguments
    ///
    /// * `build_id` - The ID of the build to stream logs for
    ///
    /// # Returns
    ///
    /// Returns a stream that yields log entries as they are received from the server.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cloud_sdk::{Client, images::ImagesClient};
    /// use futures::StreamExt;
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     let mut stream = images_client.stream_logs("build-123").await?;
    ///     while let Some(log_entry) = stream.next().await {
    ///         match log_entry {
    ///             Ok(entry) => println!("Log: {:?}", entry),
    ///             Err(e) => eprintln!("Error: {:?}", e),
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn stream_logs(
        &self,
        build_id: &str,
    ) -> Result<impl Stream<Item = Result<LogEntry, SdkError>>, SdkError> {
        let uri_str = format!("/images/v2/builds/{build_id}/logs");
        let request = self
            .client
            .request(Method::GET, &uri_str)
            .header(ACCEPT, "text/event-stream")
            .build()?;

        let response = self.client.execute(request).await?;

        let decoder: SseDecoder<LogEntry> = SseDecoder::new();
        let stream = response.bytes_stream();

        let frame = FramedRead::new(StreamReader::new(stream.map_err(Error::other)), decoder);

        Ok(frame.into_stream())
    }

    /// Calculate a simple image hash for the build request.
    fn calculate_image_hash(&self, request: &ImageBuildRequest) -> String {
        let mut hasher = Sha256::new();
        hasher.update(request.image_name.as_bytes());
        hasher.update(request.image_tag.as_bytes());
        hasher.update(&request.context_data);
        hex::encode(hasher.finalize())
    }
}
