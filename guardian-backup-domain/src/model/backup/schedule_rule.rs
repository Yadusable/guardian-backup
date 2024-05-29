use serde::{Deserialize, Serialize};
use crate::model::duration::Duration;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct ScheduleRule {
    snapshot_lifetime: Duration,
    interval: Duration,
}

impl ScheduleRule {
    pub fn new(lifetime: Duration, interval: Duration) -> Self {
        Self { snapshot_lifetime: lifetime, interval }
    }

    pub fn lifetime(&self) -> Duration {
        self.snapshot_lifetime
    }
    pub fn interval(&self) -> Duration {
        self.interval
    }
}
