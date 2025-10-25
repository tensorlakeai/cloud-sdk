use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
    pub outcome: models::FunctionRunOutcome,
}

impl Allocation {
    pub fn new(
        attempt_number: i32,
        created_at: i32,
        executor_id: String,
        function_executor_id: String,
        function_name: String,
        id: String,
        outcome: models::FunctionRunOutcome,
    ) -> Allocation {
        Allocation {
            attempt_number,
            created_at,
            execution_duration_ms: None,
            executor_id,
            function_executor_id,
            function_name,
            id,
            outcome,
        }
    }
}
