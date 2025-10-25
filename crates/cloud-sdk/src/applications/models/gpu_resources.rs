use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpuResources {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "model")]
    pub model: String,
}

impl GpuResources {
    pub fn new(count: i32, model: String) -> GpuResources {
        GpuResources { count, model }
    }
}
