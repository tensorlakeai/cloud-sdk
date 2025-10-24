use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionRunOutcome {
    #[serde(rename = "Undefined")]
    Undefined,
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Failure")]
    Failure,
}

impl std::fmt::Display for FunctionRunOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Undefined => write!(f, "Undefined"),
            Self::Success => write!(f, "Success"),
            Self::Failure => write!(f, "Failure"),
        }
    }
}

impl Default for FunctionRunOutcome {
    fn default() -> FunctionRunOutcome {
        Self::Undefined
    }
}
