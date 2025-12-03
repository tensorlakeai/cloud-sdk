use derive_builder::Builder;
use futures::Stream;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, pin::Pin};
use uuid::Uuid;

use crate::error::SdkError;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
pub struct ApplicationManifest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into), default)]
    pub description: String,
    #[builder(setter(into), default)]
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
pub struct FunctionManifest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into), default)]
    pub description: String,
    #[builder(default)]
    pub is_api: bool,
    #[builder(setter(into, strip_option), default)]
    pub secret_names: Vec<String>,
    #[builder(default)]
    pub initialization_timeout_sec: i32,
    #[builder(default)]
    pub timeout_sec: i32,
    pub resources: Resources,
    #[builder(default)]
    pub retry_policy: RetryPolicy,
    #[builder(setter(into, strip_option), default)]
    pub cache_key: Option<String>,
    #[builder(setter(into), default)]
    pub parameters: Vec<Parameter>,
    pub return_type: serde_json::Value,
    #[builder(default)]
    pub placement_constraints: PlacementConstraintsManifest,
    #[builder(default)]
    pub max_concurrency: i32,
}

impl FunctionManifest {
    pub fn builder() -> FunctionManifestBuilder {
        FunctionManifestBuilder::default()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
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
pub struct PlacementConstraintsManifest {
    #[builder(setter(into), default)]
    pub filter_expressions: Vec<String>,
}

impl PlacementConstraintsManifest {
    pub fn builder() -> PlacementConstraintsManifestBuilder {
        PlacementConstraintsManifestBuilder::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Builder)]
pub struct DataType {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub typ: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub items: Option<Box<DataType>>,
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(setter(into, strip_option), default)]
    pub additional_properties: Option<Box<DataType>>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub any_of: Option<Vec<DataType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub default_value: Option<serde_json::Value>,
}

impl DataType {
    pub fn builder() -> DataTypeBuilder {
        DataTypeBuilder::default()
    }

    pub fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Builder)]
pub struct Parameter {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into, strip_option), default)]
    pub description: Option<String>,
    #[builder(setter(into), default = "true")]
    pub required: bool,
    #[builder(setter(into))]
    pub data_type: DataType,
}

impl Parameter {
    pub fn builder() -> ParameterBuilder {
        ParameterBuilder::default()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    pub attempt_number: i32,
    pub created_at: u128,
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
    pub entrypoint: EntryPointManifest,
    pub functions: HashMap<String, ApplicationFunction>,
    pub name: String,
    pub namespace: String,
    pub tags: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tombstoned: Option<bool>,
    #[serde(skip_serializing)]
    pub state: ApplicationState,
    pub version: String,
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApplicationState {
    #[default]
    Active,
    Disabled {
        reason: String,
    },
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
    pub return_type: Option<serde_json::Value>,
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
    pub input_serializer: String,
    pub output_serializer: String,
    pub output_type_hints_base64: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionResources {
    pub cpus: f64,
    pub gpus: Vec<GpuResources>,
    pub memory_mb: i64,
    pub ephemeral_disk_mb: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRun {
    pub created_at: u128,
    pub id: String,
    pub name: String,
    pub namespace: String,
    pub application: String,
    pub application_version: String,
    pub allocations: Vec<Allocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<FunctionRunOutcome>,
    pub status: FunctionRunStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionRunOutcome {
    #[serde(alias = "Unknown")]
    Unknown,
    #[serde(alias = "Undefined")]
    Undefined,
    #[serde(alias = "Success")]
    Success,
    #[serde(alias = "Failure")]
    Failure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FunctionRunStatus {
    #[serde(alias = "Pending")]
    Pending,
    #[serde(alias = "Enqueued")]
    Enqueued,
    #[serde(alias = "Running")]
    Running,
    #[serde(alias = "Completed")]
    Completed,
    #[serde(alias = "Failed")]
    Failed,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuResources {
    pub count: u32,
    pub model: String,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRetryPolicy {
    pub max_retries: i32,
    pub initial_delay_sec: f64,
    pub max_delay_sec: f64,
    pub delay_multiplier: f64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterMetadata {
    pub data_type: serde_json::Value,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<RequestOutcome>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "failureReason")]
    pub failure_reason: Option<RequestFailureReason>,
    #[serde(alias = "applicationVersion")]
    pub application_version: String,
    #[serde(alias = "createdAt")]
    pub created_at: u128,
    #[serde(skip_serializing_if = "Option::is_none", alias = "requestError")]
    pub request_error: Option<RequestError>,
    #[serde(alias = "functionRuns")]
    pub function_runs: Vec<FunctionRun>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        default,
        alias = "progressUpdates"
    )]
    pub progress_updates: Vec<RequestStateChangeEvent>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        alias = "updatesPaginationToken"
    )]
    pub updates_pagination_token: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestError {
    pub function_name: String,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RequestFailureReason {
    #[serde(alias = "unknown")]
    Unknown,
    #[serde(alias = "internalerror", alias = "internal_error")]
    InternalError,
    #[serde(alias = "functionerror", alias = "function_error")]
    FunctionError,
    #[serde(alias = "requesterror", alias = "request_error")]
    RequestError,
    #[serde(alias = "nextfunctionnotfound", alias = "next_function_not_found")]
    NextFunctionNotFound,
    #[serde(alias = "constraintunsatisfiable", alias = "constraint_unsatisfiable")]
    ConstraintUnsatisfiable,
    #[serde(alias = "functiontimeout", alias = "function_timeout")]
    FunctionTimeout,
    #[serde(alias = "cancelled")]
    Cancelled,
    #[serde(alias = "outofmemory", alias = "out_of_memory")]
    OutOfMemory,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestOutcome {
    #[default]
    Unknown,
    Success,
    Failure(RequestFailureReason),
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShallowRequest {
    pub created_at: i64,
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSignal {
    pub timestamp: u64,
    pub uuid: Uuid,
    pub namespace: String,
    pub application: String,
    #[serde(rename = "resourceAttributes")]
    pub resource_attributes: Vec<(String, String)>,
    pub body: String,
    #[serde(rename = "logAttributes")]
    pub log_attributes: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsResponse {
    pub logs: Vec<LogSignal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestStateChangeEvent {
    RequestStarted(RequestStartedEvent),
    FunctionRunCreated(FunctionRunCreated),
    FunctionRunAssigned(FunctionRunAssigned),
    FunctionRunCompleted(FunctionRunCompleted),
    FunctionRunMatchedCache(FunctionRunMatchedCache),
    RequestCreated(RequestCreatedEvent),
    RequestProgressUpdated(RequestProgressUpdated),
    RequestFinished(RequestFinishedEvent),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringKind {
    String(String),
    Unknown(serde_json::Value),
}

impl StringKind {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            StringKind::String(value) => Some(value),
            _ => None,
        }
    }
}

impl Default for StringKind {
    fn default() -> Self {
        StringKind::String(String::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FloatKind {
    Float(f64),
    String(String),
    Unknown(serde_json::Value),
}

impl FloatKind {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            FloatKind::Float(value) => Some(*value),
            FloatKind::String(value) => value.parse().ok(),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct RequestProgressUpdated {
    pub request_id: String,
    #[serde(default)]
    pub function_name: String,
    #[serde(default)]
    pub message: StringKind,
    #[serde(default)]
    pub step: Option<FloatKind>,
    #[serde(default)]
    pub total: Option<FloatKind>,
    #[serde(default)]
    pub attributes: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreatedEvent {
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestFinishedEvent {
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
    pub request_id: String,
    #[serde(default)]
    pub outcome: RequestOutcome,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestStartedEvent {
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
    pub request_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunCreated {
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
    pub request_id: String,
    pub function_name: String,
    pub function_run_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunAssigned {
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
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
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
    pub request_id: String,
    pub function_name: String,
    pub function_run_id: String,
    pub allocation_id: String,
    pub outcome: FunctionRunOutcomeSummary,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionRunMatchedCache {
    pub namespace: String,
    pub application_name: String,
    pub application_version: String,
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
pub struct GetRequestRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
    #[builder(setter(into, strip_option), default)]
    pub updates_pagination_token: Option<String>,
}

impl GetRequestRequest {
    pub fn builder() -> GetRequestRequestBuilder {
        GetRequestRequestBuilder::default()
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

/// Response from invoking an application
pub enum InvokeResponse {
    /// The request ID of the invocation
    RequestId(String),
    /// A stream of progress events
    Stream(Pin<Box<dyn Stream<Item = Result<RequestStateChangeEvent, SdkError>> + Send>>),
}

#[derive(Builder, Debug)]
pub struct ListApplicationsRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(default, setter(strip_option))]
    pub limit: Option<i32>,
    #[builder(default, setter(into, strip_option))]
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

#[derive(Builder, Debug)]
pub struct GetLogsRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(default, setter(into, strip_option))]
    pub request_id: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub container_id: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub function: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub next_token: Option<String>,
    #[builder(default, setter(strip_option))]
    pub head: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub tail: Option<usize>,
    #[builder(default, setter(into, strip_option))]
    pub ignore: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub function_executor: Option<String>,
}

impl GetLogsRequest {
    pub fn builder() -> GetLogsRequestBuilder {
        GetLogsRequestBuilder::default()
    }
}

#[derive(Builder, Clone, Debug)]
pub struct ProgressUpdatesRequest {
    #[builder(setter(into))]
    pub namespace: String,
    #[builder(setter(into))]
    pub application: String,
    #[builder(setter(into))]
    pub request_id: String,
    pub mode: ProgressUpdatesRequestMode,
}

#[derive(Clone, Debug)]
pub enum ProgressUpdatesRequestMode {
    Paginated(Option<String>),
    FetchAll,
    Stream,
}

impl ProgressUpdatesRequest {
    pub fn builder() -> ProgressUpdatesRequestBuilder {
        ProgressUpdatesRequestBuilder::default()
    }
}

pub enum ProgressUpdatesResponse {
    /// A JSON object containing progress updates
    Updates(ProgressUpdates),
    /// A stream of progress events
    Stream(Pin<Box<dyn Stream<Item = Result<RequestStateChangeEvent, SdkError>> + Send>>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProgressUpdates {
    pub updates: Vec<RequestStateChangeEvent>,
    pub next_token: Option<String>,
}
