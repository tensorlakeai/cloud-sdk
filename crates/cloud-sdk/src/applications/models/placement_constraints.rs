use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementConstraints {
    /// List of label filter expressions in the format \"key=value\", \"key!=value\", etc.
    #[serde(rename = "filter_expressions", skip_serializing_if = "Option::is_none")]
    pub filter_expressions: Option<Vec<String>>,
}

impl PlacementConstraints {
    pub fn new() -> PlacementConstraints {
        PlacementConstraints {
            filter_expressions: None,
        }
    }
}
