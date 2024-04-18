use std::time::UNIX_EPOCH;
use serde::{Deserialize, Serialize};
use crate::model::backup::schedule_rule::ScheduleRule;
use crate::model::error::{GuardianError, Result};
use crate::model::timestamp::Timestamp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    rules: Vec<ScheduleRule>,
    last_execution: Timestamp,
}

impl Schedule {
    pub fn new(rules: Vec<ScheduleRule>, last_execution: Timestamp) -> Self {
        Self {
            rules,
            last_execution,
        }
    }

    pub fn rules(&self) -> &[ScheduleRule] {
        self.rules.as_slice()
    }

    pub fn add_rule(&mut self, rule: ScheduleRule) {
        self.rules.push(rule)
    }

    pub fn remove_rule(&mut self, rule: &ScheduleRule) -> Result<()> {
        let prev_size = self.rules.len();
        self.rules.retain(|e| e != rule);
        if prev_size == self.rules.len() {
            return Err(GuardianError::InternalError {
                message: "Tried to remove non existing rule from schedule".into(),
            });
        }
        Ok(())
    }

    pub fn last_execution(&self) -> Timestamp {
        self.last_execution
    }

    pub fn set_last_execution(&mut self, last_execution: Timestamp) {
        self.last_execution = last_execution;
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new(vec![], Timestamp::from_milliseconds(UNIX_EPOCH.elapsed().unwrap().as_millis() as u64))
    }
}
