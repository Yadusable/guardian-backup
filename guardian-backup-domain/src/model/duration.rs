use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Duration {
    Infinite,
    Limited { milliseconds: u64 },
}
