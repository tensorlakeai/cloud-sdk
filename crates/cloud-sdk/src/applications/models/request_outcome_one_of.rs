use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestOutcomeOneOf {
    #[serde(rename = "failure")]
    pub failure: models::RequestFailureReason,
}

impl RequestOutcomeOneOf {
    pub fn new(failure: models::RequestFailureReason) -> RequestOutcomeOneOf {
        RequestOutcomeOneOf { failure }
    }
}
