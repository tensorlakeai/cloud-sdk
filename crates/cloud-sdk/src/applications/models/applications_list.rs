use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationsList {
    #[serde(rename = "applications")]
    pub applications: Vec<models::Application>,
    #[serde(
        rename = "cursor",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cursor: Option<Option<String>>,
}

impl ApplicationsList {
    pub fn new(applications: Vec<models::Application>) -> ApplicationsList {
        ApplicationsList {
            applications,
            cursor: None,
        }
    }
}
