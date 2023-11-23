use crate::model::duration::Duration;

pub struct ScheduleRule {
    lifetime: Duration,
    interval: Duration,
}