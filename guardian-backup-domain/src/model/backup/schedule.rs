use crate::model::backup::schedule_rule::ScheduleRule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Schedule {
    rules: Vec<ScheduleRule>,
}

impl Schedule {
    pub fn new(rules: Vec<ScheduleRule>) -> Self {
        Self { rules }
    }

    pub fn rules(&self) -> &[ScheduleRule] {
        self.rules.as_slice()
    }

    pub fn add_rule(&mut self, rule: ScheduleRule) {
        self.rules.push(rule)
    }

    pub fn remove_rule(&mut self, rule: &ScheduleRule) -> Result<(), ScheduleError> {
        let prev_size = self.rules.len();
        self.rules.retain(|e| e != rule);
        if prev_size == self.rules.len() {
            return Err(ScheduleError::RuleNotInSchedule);
        }
        Ok(())
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new(vec![])
    }
}

pub enum ScheduleError {
    RuleNotInSchedule,
}
