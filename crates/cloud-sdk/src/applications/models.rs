use derive_builder::Builder;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
pub struct ApplicationManifest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub description: String,
    #[builder(setter(into))]
    pub tags: HashMap<String, String>,
    #[builder(setter(into))]
    pub version: String,
    pub functions: HashMap<String, FunctionManifest>,
    pub entrypoint: Entrypoint,
}

impl ApplicationManifest {
    pub fn builder() -> ApplicationManifestBuilder {
        ApplicationManifestBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(name = "EntrypointBuilder")]
pub struct Entrypoint {
    #[builder(setter(into))]
    pub function_name: String,
    #[builder(setter(into))]
    pub input_serializer: String,
    #[builder(setter(into))]
    pub output_serializer: String,
    #[builder(setter(into, strip_option), default)]
    pub output_type_hints_base64: Option<String>,
}

impl Entrypoint {
    pub fn builder() -> EntrypointBuilder {
        EntrypointBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(name = "FunctionManifestBuilder")]
pub struct FunctionManifest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub description: String,
    pub is_api: bool,
    #[builder(setter(into, strip_option), default)]
    pub secret_names: Vec<String>,
    pub initialization_timeout_sec: i32,
    pub timeout_sec: i32,
    pub resources: Resources,
    pub retry_policy: RetryPolicy,
    #[builder(setter(into, strip_option), default)]
    pub cache_key: Option<String>,
    #[builder(setter(into), default)]
    pub parameters: Vec<Parameter>,
    pub return_type: serde_json::Value,
    pub placement_constraints: PlacementConstraintsManifest,
    pub max_concurrency: i32,
}

impl FunctionManifest {
    pub fn builder() -> FunctionManifestBuilder {
        FunctionManifestBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(name = "ResourcesBuilder")]
pub struct Resources {
    pub cpus: f64,
    pub memory_mb: i64,
    pub ephemeral_disk_mb: i64,
    #[builder(setter(into), default)]
    pub gpus: Vec<String>,
}

impl Resources {
    pub fn builder() -> ResourcesBuilder {
        ResourcesBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(name = "RetryPolicyBuilder")]
pub struct RetryPolicy {
    pub max_retries: i32,
    pub initial_delay_sec: f64,
    pub max_delay_sec: f64,
    pub delay_multiplier: f64,
}

impl RetryPolicy {
    pub fn builder() -> RetryPolicyBuilder {
        RetryPolicyBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(name = "PlacementConstraintsManifestBuilder")]
pub struct PlacementConstraintsManifest {
    #[builder(setter(into), default)]
    pub filter_expressions: Vec<String>,
}

impl PlacementConstraintsManifest {
    pub fn builder() -> PlacementConstraintsManifestBuilder {
        PlacementConstraintsManifestBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(name = "ParameterBuilder")]
pub struct Parameter {
    #[builder(setter(into))]
    pub name: String,
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub param_type: String,
}

impl Parameter {
    pub fn builder() -> ParameterBuilder {
        ParameterBuilder::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    pub attempt_number: i32,
    pub created_at: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_duration_ms: Option<i64>,
    pub executor_id: String,
    pub function_executor_id: String,
    pub function_name: String,
    pub id: String,
    pub outcome: FunctionRunOutcome,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    pub description: String,
    pub entrypoint: Box<EntryPointManifest>,
    pub functions: HashMap<String, ApplicationFunction>,
    pub name: String,
    pub namespace: String,
    pub tags: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tombstoned: Option<bool>,
    pub version: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationFunction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_timeout_sec: Option<i32>,
    pub max_concurrency: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterMetadata>>,
    pub placement_constraints: Box<PlacementConstraints>,
    pub resources: Box<FunctionResources>,
    pub retry_policy: Box<NodeRetryPolicy>,
    #[serde(
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_type: Option<Option<serde_json::Value>>,
    pub secret_names: Vec<String>,
    pub timeout_sec: i32,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRequests {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub requests: Vec<ShallowRequest>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationsList {
    pub applications: Vec<Application>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CursorDirection {
    Forward,
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
    pub content_length: Option<HeaderValue>,
    pub content_type: Option<HeaderValue>,
    pub content: bytes::Bytes,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntryPointManifest {
    pub function_name: String,
    pub name: String,
    pub version: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionResources {
    pub cpu_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpus: Option<Vec<GpuResources>>,
    pub memory_bytes: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_bytes: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRun {
    pub allocations: Vec<Allocation>,
    pub created_at: i64,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Option<FunctionRunOutcome>>,
    pub status: FunctionRunStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionRunOutcome {
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionRunStatus {
    Pending,
    Enqueued,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuResources {
    pub count: i32,
    pub model: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRetryPolicy {
    pub backoff_multiplier: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_multiplier: Option<f64>,
    pub max_retries: i32,
    pub timeout_secs: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterMetadata {
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    pub required: bool,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementConstraints {
    /// List of label filter expressions in the format "key=value", "key!=value", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub created_at: i64,
    pub function_runs: Vec<FunctionRun>,
    pub id: String,
    pub outcome: RequestOutcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_error: Option<Option<Box<RequestError>>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestError {
    pub function_name: String,
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
    pub failure: RequestFailureReason,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShallowRequest {
    pub created_at: i64,
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestStateChangeEvent {
    RequestStarted(RequestStartedEvent),
    FunctionRunCreated(FunctionRunCreated),
    FunctionRunAssigned(FunctionRunAssigned),
    FunctionRunCompleted(FunctionRunCompleted),
    FunctionRunMatchedCache(FunctionRunMatchedCache),
    RequestCreated(RequestCreatedEvent),
    RequestFinished(RequestFinishedEvent),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreatedEvent {
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestFinishedEvent {
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestStartedEvent {
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunCreated {
    pub request_id: String,
    pub function_name: String,
    pub function_run_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunAssigned {
    pub request_id: String,
    pub function_name: String,
    pub function_run_id: String,
    pub allocation_id: String,
    pub executor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FunctionRunOutcomeSummary {
    Unknown,
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunCompleted {
    pub request_id: String,
    pub function_name: String,
    pub function_run_id: String,
    pub allocation_id: String,
    pub outcome: FunctionRunOutcomeSummary,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunMatchedCache {
    pub request_id: String,
    pub function_name: String,
    pub function_run_id: String,
}

#[derive(Builder, Debug)]
pub struct CheckFunctionOutputRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
}

impl CheckFunctionOutputRequest {
    pub fn builder() -> CheckFunctionOutputRequestBuilder {
        CheckFunctionOutputRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct DeleteApplicationRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
}

impl DeleteApplicationRequest {
    pub fn builder() -> DeleteApplicationRequestBuilder {
        DeleteApplicationRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct DeleteFunctionRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub function_name: String,
}

impl DeleteFunctionRequest {
    pub fn builder() -> DeleteFunctionRequestBuilder {
        DeleteFunctionRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct DeleteRequestRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
}

impl DeleteRequestRequest {
    pub fn builder() -> DeleteRequestRequestBuilder {
        DeleteRequestRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct DownloadFunctionOutputRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
    #[builder(setter(into))]
    pub function_call_id: String,
}

impl DownloadFunctionOutputRequest {
    pub fn builder() -> DownloadFunctionOutputRequestBuilder {
        DownloadFunctionOutputRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct DownloadRequestOutputRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
}

impl DownloadRequestOutputRequest {
    pub fn builder() -> DownloadRequestOutputRequestBuilder {
        DownloadRequestOutputRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct GetApplicationRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
}

impl GetApplicationRequest {
    pub fn builder() -> GetApplicationRequestBuilder {
        GetApplicationRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct InvokeApplicationRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    pub body: serde_json::Value,
    #[builder(default)]
    pub stream: bool,
}

impl InvokeApplicationRequest {
    pub fn builder() -> InvokeApplicationRequestBuilder {
        InvokeApplicationRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct ListApplicationsRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(default, setter(strip_option))]
    pub limit: Option<i32>,
    #[builder(default, setter(strip_option))]
    pub cursor: Option<String>,
    #[builder(default, setter(strip_option))]
    pub direction: Option<CursorDirection>,
}

impl ListApplicationsRequest {
    pub fn builder() -> ListApplicationsRequestBuilder {
        ListApplicationsRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct ListRequestsRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(default, setter(strip_option))]
    pub limit: Option<i32>,
    #[builder(default, setter(into, strip_option))]
    pub cursor: Option<String>,
    #[builder(default, setter(strip_option))]
    pub direction: Option<CursorDirection>,
}

impl ListRequestsRequest {
    pub fn builder() -> ListRequestsRequestBuilder {
        ListRequestsRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct StreamProgressRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
}

impl StreamProgressRequest {
    pub fn builder() -> StreamProgressRequestBuilder {
        StreamProgressRequestBuilder::default()
    }
}

#[derive(Builder, Debug)]
pub struct UpsertApplicationRequest {
    #[builder(setter(into))]
    pub namespace: String,
    pub application_manifest: ApplicationManifest,
    #[builder(setter(into))]
    pub code_zip: Vec<u8>,
}

impl UpsertApplicationRequest {
    pub fn builder() -> UpsertApplicationRequestBuilder {
        UpsertApplicationRequestBuilder::default()
    }
}
