use derive_builder::Builder;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
pub struct ApplicationManifest {
    #[serde(rename = "name")]
    #[builder(setter(into))]
    pub name: String,
    #[serde(rename = "description")]
    #[builder(setter(into))]
    pub description: String,
    #[serde(rename = "tags")]
    #[builder(setter(into))]
    pub tags: HashMap<String, String>,
    #[serde(rename = "version")]
    #[builder(setter(into))]
    pub version: String,
    #[serde(rename = "functions")]
    pub functions: HashMap<String, FunctionManifest>,
    #[serde(rename = "entrypoint")]
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
    #[serde(rename = "function_name")]
    #[builder(setter(into))]
    pub function_name: String,
    #[serde(rename = "input_serializer")]
    #[builder(setter(into))]
    pub input_serializer: String,
    #[serde(rename = "output_serializer")]
    #[builder(setter(into))]
    pub output_serializer: String,
    #[serde(rename = "output_type_hints_base64")]
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
    #[serde(rename = "name")]
    #[builder(setter(into))]
    pub name: String,
    #[serde(rename = "description")]
    #[builder(setter(into))]
    pub description: String,
    #[serde(rename = "is_api")]
    pub is_api: bool,
    #[serde(rename = "secret_names")]
    #[builder(setter(into, strip_option), default)]
    pub secret_names: Vec<String>,
    #[serde(rename = "initialization_timeout_sec")]
    pub initialization_timeout_sec: i32,
    #[serde(rename = "timeout_sec")]
    pub timeout_sec: i32,
    #[serde(rename = "resources")]
    pub resources: Resources,
    #[serde(rename = "retry_policy")]
    pub retry_policy: RetryPolicy,
    #[serde(rename = "cache_key", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub cache_key: Option<String>,
    #[serde(rename = "parameters")]
    #[builder(setter(into), default)]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "return_type")]
    pub return_type: serde_json::Value,
    #[serde(rename = "placement_constraints")]
    pub placement_constraints: PlacementConstraintsManifest,
    #[serde(rename = "max_concurrency")]
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
    #[serde(rename = "cpus")]
    pub cpus: f64,
    #[serde(rename = "memory_mb")]
    pub memory_mb: i64,
    #[serde(rename = "ephemeral_disk_mb")]
    pub ephemeral_disk_mb: i64,
    #[serde(rename = "gpus")]
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
    #[serde(rename = "max_retries")]
    pub max_retries: i32,
    #[serde(rename = "initial_delay_sec")]
    pub initial_delay_sec: f64,
    #[serde(rename = "max_delay_sec")]
    pub max_delay_sec: f64,
    #[serde(rename = "delay_multiplier")]
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
    #[serde(rename = "filter_expressions")]
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
    #[serde(rename = "name")]
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
    #[serde(rename = "attempt_number")]
    pub attempt_number: i32,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(
        rename = "execution_duration_ms",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_duration_ms: Option<i64>,
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
    pub content_length: Option<HeaderValue>,
    pub content_type: Option<HeaderValue>,
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
