use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CursorDirection {
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "backward")]
    Backward,
}

impl std::fmt::Display for CursorDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Forward => write!(f, "forward"),
            Self::Backward => write!(f, "backward"),
        }
    }
}

impl Default for CursorDirection {
    fn default() -> CursorDirection {
        Self::Forward
    }
}
