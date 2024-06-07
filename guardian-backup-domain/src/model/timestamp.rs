use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    milliseconds_since_epoch: u64,
}

impl Timestamp {
    pub fn from_milliseconds(millis: u64) -> Self {
        Self {
            milliseconds_since_epoch: millis,
        }
    }

    #[cfg(not(test))]
    pub fn now() -> Self {
        Self {
            milliseconds_since_epoch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    #[cfg(test)]
    pub fn now() -> Self {
        Self {
            milliseconds_since_epoch: 25569,
        }
    }

    pub fn from_now_in_millis(diff_in_millis: u64) -> Self {
        Self {
            milliseconds_since_epoch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
                + diff_in_millis,
        }
    }
}
