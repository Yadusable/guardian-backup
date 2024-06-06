use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Duration {
    Infinite,
    Limited { milliseconds: u64 },
}

impl Duration {
    pub fn get_duration(self) -> Option<u64> {
        match self {
            Duration::Infinite => None,
            Duration::Limited { milliseconds } => Some(milliseconds),
        }
    }
}
