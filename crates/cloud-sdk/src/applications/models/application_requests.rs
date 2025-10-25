use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationRequests {
    #[serde(
        rename = "next_cursor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<Option<String>>,
    #[serde(
        rename = "prev_cursor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub prev_cursor: Option<Option<String>>,
    #[serde(rename = "requests")]
    pub requests: Vec<models::ShallowRequest>,
}

impl ApplicationRequests {
    pub fn new(requests: Vec<models::ShallowRequest>) -> ApplicationRequests {
        ApplicationRequests {
            next_cursor: None,
            prev_cursor: None,
            requests,
        }
    }
}
