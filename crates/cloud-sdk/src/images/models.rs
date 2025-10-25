use serde::{Deserialize, Serialize};

/// Request parameters for building an image.
#[derive(Debug, Clone)]
pub struct ImageBuildRequest {
    /// The name of the image to build.
    pub image_name: String,
    /// The tag for the image.
    pub image_tag: String,
    /// The build context data as a tar.gz archive.
    pub context_data: Vec<u8>,
    /// The name of the application this image belongs to.
    pub application_name: String,
    /// The version of the application.
    pub application_version: String,
    /// The name of the function in the application.
    pub function_name: String,
}

/// Result of an image build operation.
#[derive(Debug, Clone)]
pub struct ImageBuildResult {
    /// The unique ID of the build.
    pub id: String,
    /// The final status of the build.
    pub status: BuildStatus,
    /// When the build was created.
    pub created_at: String,
    /// When the build finished (if completed).
    pub finished_at: Option<String>,
    /// Error message if the build failed.
    pub error_message: Option<String>,
}

/// The status of an image build.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildStatus {
    /// The build is pending.
    Pending,
    /// The build is enqueued.
    Enqueued,
    /// The build is in progress.
    Building,
    /// The build completed successfully.
    Succeeded,
    /// The build failed.
    Failed,
    /// The build is being canceled.
    Canceling,
    /// The build was canceled.
    Canceled,
}

/// Registry type for the image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryType {
    /// ECR registry.
    ECR,
    /// Docker registry.
    Docker,
}

/// Response for listing builds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildListResponse {
    /// The public ID of the build.
    pub public_id: String,
    /// The name of the image.
    pub name: String,
    /// Tags associated with the build.
    pub tags: Vec<String>,
    /// The creation time of the build.
    pub creation_time: String,
    /// The status of the build.
    pub status: BuildStatus,
}

/// Paginated page of build list responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// The total number of items.
    pub total_items: i64,
    /// The current page number.
    pub page: i32,
    /// The number of items per page.
    pub page_size: i32,
    /// The total number of pages.
    pub total_pages: i32,
}

/// Response for canceling a build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelBuildResponse {
    /// The status message.
    pub status: String,
}

/// Log entry for streaming logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// The build ID.
    pub build_id: String,
    /// The timestamp of the log entry.
    pub timestamp: String,
    /// The stream type.
    pub stream: String,
    /// The log message.
    pub message: String,
    /// The sequence number.
    pub sequence_number: i64,
    /// The build status at the time of the log.
    pub build_status: String,
}

/// Response for build info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInfoResponse {
    /// The build ID.
    pub id: String,
    /// The build status.
    pub status: BuildStatus,
    /// Error message if failed.
    pub error_message: Option<String>,
    /// Creation time.
    pub created_at: String,
    /// Updated time.
    pub updated_at: String,
    /// Finished time.
    pub finished_at: Option<String>,
    /// Image hash.
    pub image_hash: String,
    /// Image name.
    pub image_name: Option<String>,
}

/// Response for pulling an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePullResponse {
    /// The build ID.
    pub id: String,
    /// The image URI.
    pub image_uri: String,
    /// The image hash.
    pub image_hash: String,
    /// The image digest.
    pub image_digest: String,
    /// The image name.
    pub image_name: String,
    /// The registry type.
    pub registry: RegistryType,
    /// The build status.
    pub status: BuildStatus,
    /// Error message if failed.
    pub error: Option<String>,
    /// Creation time.
    pub created_at: String,
    /// Finished time.
    pub finished_at: Option<String>,
}

/// Internal representation of build information from the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildInfo {
    pub id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub finished_at: Option<String>,
    pub error_message: Option<String>,
}
