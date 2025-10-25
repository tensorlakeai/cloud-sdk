use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionRun {
    #[serde(rename = "allocations")]
    pub allocations: Vec<models::Allocation>,
    #[serde(rename = "application")]
    pub application: String,
    #[serde(rename = "application_version")]
    pub application_version: String,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(
        rename = "outcome",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub outcome: Option<Option<models::FunctionRunOutcome>>,
    #[serde(rename = "status")]
    pub status: models::FunctionRunStatus,
}

impl FunctionRun {
    pub fn new(
        allocations: Vec<models::Allocation>,
        application: String,
        application_version: String,
        created_at: i32,
        id: String,
        name: String,
        namespace: String,
        status: models::FunctionRunStatus,
    ) -> FunctionRun {
        FunctionRun {
            allocations,
            application,
            application_version,
            created_at,
            id,
            name,
            namespace,
            outcome: None,
            status,
        }
    }
}
