use crate::applications::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionResources {
    #[serde(rename = "cpus")]
    pub cpus: f64,
    #[serde(rename = "ephemeral_disk_mb")]
    pub ephemeral_disk_mb: i64,
    #[serde(rename = "gpus", skip_serializing_if = "Option::is_none")]
    pub gpus: Option<Vec<models::GpuResources>>,
    #[serde(rename = "memory_mb")]
    pub memory_mb: i64,
}

impl FunctionResources {
    pub fn new(cpus: f64, ephemeral_disk_mb: i64, memory_mb: i64) -> FunctionResources {
        FunctionResources {
            cpus,
            ephemeral_disk_mb,
            gpus: None,
            memory_mb,
        }
    }
}
