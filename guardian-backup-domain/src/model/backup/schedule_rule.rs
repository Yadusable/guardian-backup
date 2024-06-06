use crate::model::duration::Duration;
use crate::model::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct ScheduleRule {
    snapshot_lifetime: Duration,
    interval: Duration,
    last_execution: Timestamp,
}

impl ScheduleRule {
    pub fn lifetime(&self) -> Duration {
        self.snapshot_lifetime
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn new(snapshot_lifetime: Duration, interval: Duration, last_execution: Timestamp) -> Self {
        Self {
            snapshot_lifetime,
            interval,
            last_execution,
        }
    }

    pub fn last_execution(&self) -> Timestamp {
        self.last_execution
    }

    pub fn set_last_execution(&mut self, last_execution: Timestamp) {
        self.last_execution = last_execution;
    }
}
