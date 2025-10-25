use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    #[serde(rename = "attempt_number")]
    pub attempt_number: i32,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(
        rename = "execution_duration_ms",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_duration_ms: Option<Option<i64>>,
    #[serde(rename = "executor_id")]
    pub executor_id: String,
    #[serde(rename = "function_executor_id")]
    pub function_executor_id: String,
    #[serde(rename = "function_name")]
    pub function_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "outcome")]
    pub outcome: FunctionRunOutcome,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "entrypoint")]
    pub entrypoint: Box<EntryPointManifest>,
    #[serde(rename = "functions")]
    pub functions: HashMap<String, ApplicationFunction>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "tags")]
    pub tags: HashMap<String, String>,
    #[serde(rename = "tombstoned", skip_serializing_if = "Option::is_none")]
    pub tombstoned: Option<bool>,
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationFunction {
    #[serde(rename = "cache_key", skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(
        rename = "initialization_timeout_sec",
        skip_serializing_if = "Option::is_none"
    )]
    pub initialization_timeout_sec: Option<i32>,
    #[serde(rename = "max_concurrency")]
    pub max_concurrency: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterMetadata>>,
    #[serde(rename = "placement_constraints")]
    pub placement_constraints: Box<PlacementConstraints>,
    #[serde(rename = "resources")]
    pub resources: Box<FunctionResources>,
    #[serde(rename = "retry_policy")]
    pub retry_policy: Box<NodeRetryPolicy>,
    #[serde(
        rename = "return_type",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_type: Option<Option<serde_json::Value>>,
    #[serde(rename = "secret_names")]
    pub secret_names: Vec<String>,
    #[serde(rename = "timeout_sec")]
    pub timeout_sec: i32,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRequests {
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "requests")]
    pub requests: Vec<ShallowRequest>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationsList {
    #[serde(rename = "applications")]
    pub applications: Vec<Application>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CursorDirection {
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "backward")]
    Backward,
}

impl std::fmt::Display for CursorDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorDirection::Forward => write!(f, "forward"),
            CursorDirection::Backward => write!(f, "backward"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DownloadOutput {
    pub content_length: Option<reqwest::header::HeaderValue>,
    pub content_type: Option<reqwest::header::HeaderValue>,
    pub content: bytes::Bytes,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntryPointManifest {
    #[serde(rename = "function_name")]
    pub function_name: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionResources {
    #[serde(rename = "cpu_count")]
    pub cpu_count: i32,
    #[serde(rename = "gpus", skip_serializing_if = "Option::is_none")]
    pub gpus: Option<Vec<GpuResources>>,
    #[serde(rename = "memory_bytes")]
    pub memory_bytes: i64,
    #[serde(rename = "storage_bytes", skip_serializing_if = "Option::is_none")]
    pub storage_bytes: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRun {
    #[serde(rename = "allocations")]
    pub allocations: Vec<Allocation>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "outcome", skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Option<FunctionRunOutcome>>,
    #[serde(rename = "status")]
    pub status: FunctionRunStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionRunOutcome {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionRunStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "enqueued")]
    Enqueued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuResources {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "model")]
    pub model: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRetryPolicy {
    #[serde(rename = "backoff_multiplier")]
    pub backoff_multiplier: f64,
    #[serde(rename = "delay_multiplier", skip_serializing_if = "Option::is_none")]
    pub delay_multiplier: Option<f64>,
    #[serde(rename = "max_retries")]
    pub max_retries: i32,
    #[serde(rename = "timeout_secs")]
    pub timeout_secs: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterMetadata {
    #[serde(rename = "data_type")]
    pub data_type: String,
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "required")]
    pub required: bool,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementConstraints {
    /// List of label filter expressions in the format "key=value", "key!=value", etc.
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "function_runs")]
    pub function_runs: Vec<FunctionRun>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "outcome")]
    pub outcome: RequestOutcome,
    #[serde(rename = "request_error", skip_serializing_if = "Option::is_none")]
    pub request_error: Option<Option<Box<RequestError>>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestError {
    #[serde(rename = "function_name")]
    pub function_name: String,
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestFailureReason {
    #[serde(rename = "requesterror")]
    Requesterror,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestOutcome {
    Success(serde_json::Value),
    Failure(RequestOutcomeOneOf),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestOutcomeOneOf {
    #[serde(rename = "failure")]
    pub failure: RequestFailureReason,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShallowRequest {
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "id")]
    pub id: String,
}
