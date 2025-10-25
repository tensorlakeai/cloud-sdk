use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestOutcome {
    String(String),
    RequestOutcomeOneOf(Box<models::RequestOutcomeOneOf>),
}

impl Default for RequestOutcome {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
