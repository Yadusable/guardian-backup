use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    milliseconds_since_epoch: u64,
}

impl Timestamp {
    pub fn from_milliseconds(millis: u64) -> Self {
        Self{
            milliseconds_since_epoch: millis,
        }
    }
}
