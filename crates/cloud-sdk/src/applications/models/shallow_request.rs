use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShallowRequest {
    #[serde(rename = "application_version")]
    pub application_version: String,
    #[serde(rename = "created_at")]
    pub created_at: i32,
    #[serde(rename = "function_runs_count")]
    pub function_runs_count: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "outcome",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub outcome: Option<Option<Box<models::RequestOutcome>>>,
}

impl ShallowRequest {
    pub fn new(
        application_version: String,
        created_at: i32,
        function_runs_count: i32,
        id: String,
    ) -> ShallowRequest {
        ShallowRequest {
            application_version,
            created_at,
            function_runs_count,
            id,
            outcome: None,
        }
    }
}
