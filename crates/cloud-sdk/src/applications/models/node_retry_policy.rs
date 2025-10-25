use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeRetryPolicy {
    #[serde(rename = "delay_multiplier")]
    pub delay_multiplier: f64,
    #[serde(rename = "initial_delay_sec")]
    pub initial_delay_sec: f64,
    #[serde(rename = "max_delay_sec")]
    pub max_delay_sec: f64,
    #[serde(rename = "max_retries")]
    pub max_retries: i32,
}

impl NodeRetryPolicy {
    pub fn new(
        delay_multiplier: f64,
        initial_delay_sec: f64,
        max_delay_sec: f64,
        max_retries: i32,
    ) -> NodeRetryPolicy {
        NodeRetryPolicy {
            delay_multiplier,
            initial_delay_sec,
            max_delay_sec,
            max_retries,
        }
    }
}
