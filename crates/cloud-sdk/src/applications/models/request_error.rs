use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestError {
    #[serde(rename = "function_name")]
    pub function_name: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl RequestError {
    pub fn new(function_name: String, message: String) -> RequestError {
        RequestError {
            function_name,
            message,
        }
    }
}
