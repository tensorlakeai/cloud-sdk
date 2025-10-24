use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestFailureReason {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "internalerror")]
    Internalerror,
    #[serde(rename = "functionerror")]
    Functionerror,
    #[serde(rename = "requesterror")]
    Requesterror,
    #[serde(rename = "constraintunsatisfiable")]
    Constraintunsatisfiable,
}

impl std::fmt::Display for RequestFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Internalerror => write!(f, "internalerror"),
            Self::Functionerror => write!(f, "functionerror"),
            Self::Requesterror => write!(f, "requesterror"),
            Self::Constraintunsatisfiable => write!(f, "constraintunsatisfiable"),
        }
    }
}

impl Default for RequestFailureReason {
    fn default() -> RequestFailureReason {
        Self::Unknown
    }
}
