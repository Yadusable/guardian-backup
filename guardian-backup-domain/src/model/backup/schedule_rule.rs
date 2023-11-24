use crate::model::duration::Duration;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ScheduleRule {
    lifetime: Duration,
    interval: Duration,
}

impl ScheduleRule {
    pub fn new(lifetime: Duration, interval: Duration) -> Self {
        Self { lifetime, interval }
    }

    pub fn lifetime(&self) -> Duration {
        self.lifetime
    }
    pub fn interval(&self) -> Duration {
        self.interval
    }
}
