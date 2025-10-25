//! # Tensorlake Cloud SDK - Images
//!
//! This module provides functionality for building and managing container images
//! in the Tensorlake Cloud platform.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use cloud_sdk::{Sdk, images::models::ImageBuildRequest};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let sdk = Sdk::new("https://api.tensorlake.ai", "your-api-key")?;
//!     let images_client = sdk.images();
//!
//!     // Build an image
//!     let build_request = ImageBuildRequest {
//!         image_name: "my-app".to_string(),
//!         image_tag: "latest".to_string(),
//!         context_data: vec![/* tar.gz context data */],
//!         application_name: "my-app".to_string(),
//!         application_version: "1.0.0".to_string(),
//!         function_name: "main".to_string(),
//!     };
//!
//!     let build_result = images_client.build_image(build_request).await?;
//!     println!("Build completed: {}", build_result.id);
//!
//!     Ok(())
//! }
//! ```

use std::io::{Error, ErrorKind};

use crate::{
    client::Client,
    event_source::{self, SseDecoder},
};
use futures::{TryStreamExt, stream::Stream};
use hex;
use miette::{Context, IntoDiagnostic};
use reqwest::header::ACCEPT;
use sha2::{Digest, Sha256};
use tokio_util::{codec::FramedRead, io::StreamReader};

pub mod models;
use models::*;

/// A client for managing image builds in Tensorlake Cloud.
#[derive(Debug)]
pub struct ImagesClient {
    client: Client,
    build_service_url: String,
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
    /// ```rust,no_run
    /// use cloud_sdk::{Client, images::ImagesClient};
    ///
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("https://api.tensorlake.ai", "your-api-key")?;
    ///     let images_client = ImagesClient::new(client);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Self {
        Self {
            build_service_url: format!("{}/images/v2", client.base_url()),
            client,
        }
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
    /// ```rust,no_run
    /// use cloud_sdk::images::{ImagesClient, models::ImageBuildRequest};
    ///
    /// async fn example(images_client: &ImagesClient) -> Result<(), Box<dyn std::error::Error>> {
    ///     let request = ImageBuildRequest {
    ///         image_name: "my-image".to_string(),
    ///         image_tag: "v1.0".to_string(),
    ///         context_data: vec![/* context tar.gz data */],
    ///         application_name: "my-app".to_string(),
    ///         application_version: "1.0.0".to_string(),
    ///         function_name: "main".to_string(),
    ///     };
    ///
    ///     let result = images_client.build_image(request).await?;
    ///     println!("Build {} completed successfully", result.id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn build_image(
        &self,
        request: ImageBuildRequest,
    ) -> miette::Result<ImageBuildResult> {
        let build_info = self.submit_build_request(&request).await?;
        self.poll_build_status(&build_info.id).await
    }

    /// Submit a build request to the build service.
    async fn submit_build_request(&self, request: &ImageBuildRequest) -> miette::Result<BuildInfo> {
        let form = reqwest::multipart::Form::new()
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

        let response = self
            .client
            .put(format!("{}/builds", self.build_service_url))
            .multipart(form)
            .send()
            .await
            .into_diagnostic()
            .with_context(|| "Failed to send build request")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            miette::bail!(
                "Build request failed with status {}: {}",
                status,
                error_text
            );
        }

        response
            .json::<BuildInfo>()
            .await
            .into_diagnostic()
            .with_context(|| "Failed to parse build response")
    }

    /// Poll the build status until completion.
    async fn poll_build_status(&self, build_id: &str) -> miette::Result<ImageBuildResult> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            if attempts > 10 {
                miette::bail!("Build polling timed out after {} attempts", attempts);
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            let response = self
                .client
                .get(format!("{}/builds/{}", self.build_service_url, build_id))
                .send()
                .await
                .into_diagnostic()
                .with_context(|| format!("Failed to check build status for {}", build_id))?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                miette::bail!(
                    "Build status check failed with status {}: {}",
                    status,
                    error_text
                );
            }

            let build_info: BuildInfo =
                response.json().await.into_diagnostic().with_context(|| {
                    format!("Failed to parse build status response for {}", build_id)
                })?;

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
    /// * `graph_name` - Optional filter by graph name
    /// * `image_name` - Optional filter by image name
    /// * `graph_function_name` - Optional filter by graph function name
    ///
    /// # Returns
    ///
    /// Returns a paginated list of builds.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_builds(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        status: Option<&BuildStatus>,
        graph_name: Option<&str>,
        image_name: Option<&str>,
        graph_function_name: Option<&str>,
    ) -> miette::Result<Page<BuildListResponse>> {
        let mut url = format!("{}/builds", self.build_service_url);

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
        if let Some(gn) = graph_name {
            query_params.push(("graph_name", gn.to_string()));
        }
        if let Some(iname) = image_name {
            query_params.push(("image_name", iname.to_string()));
        }
        if let Some(gfn) = graph_function_name {
            query_params.push(("graph_function_name", gfn.to_string()));
        }

        if !query_params.is_empty() {
            url.push('?');
            let params: Vec<String> = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect();
            url.push_str(&params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .into_diagnostic()
            .with_context(|| format!("Failed to send list builds request to {}", url))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            miette::bail!(
                "List builds request failed with status {}: {}",
                status,
                error_text
            );
        }

        response
            .json::<Page<BuildListResponse>>()
            .await
            .into_diagnostic()
            .with_context(|| "Failed to parse list builds response")
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
    pub async fn cancel_build(&self, build_id: &str) -> miette::Result<()> {
        let url = format!("{}/builds/{}/cancel", self.build_service_url, build_id);

        let response = self
            .client
            .post(&url)
            .send()
            .await
            .into_diagnostic()
            .with_context(|| format!("Failed to send cancel build request to {}", url))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            miette::bail!(
                "Cancel build request failed with status {}: {}",
                status,
                error_text
            );
        }

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
    pub async fn get_build_info(&self, build_id: &str) -> miette::Result<BuildInfoResponse> {
        let url = format!("{}/builds/{}", self.build_service_url, build_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .into_diagnostic()
            .with_context(|| format!("Failed to send get build info request to {}", url))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            miette::bail!(
                "Get build info request failed with status {}: {}",
                status,
                error_text
            );
        }

        response
            .json::<BuildInfoResponse>()
            .await
            .into_diagnostic()
            .with_context(|| "Failed to parse build info response")
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
    pub async fn stream_logs(
        &self,
        build_id: &str,
    ) -> miette::Result<impl Stream<Item = Result<LogEntry, event_source::Error>>> {
        let url = format!("{}/builds/{}/logs", self.build_service_url, build_id);

        let response = self
            .client
            .get(&url)
            .header(ACCEPT, "text/event-stream")
            .send()
            .await
            .into_diagnostic()
            .with_context(|| format!("Failed to send stream logs request to {}", url))?;

        let decoder: SseDecoder<LogEntry> = SseDecoder::new();
        let stream = response.bytes_stream();

        let frame = FramedRead::new(
            StreamReader::new(stream.map_err(|e| Error::new(ErrorKind::Other, e))),
            decoder,
        );

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
