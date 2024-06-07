use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub const MONTH: Duration = Duration::Limited {
    milliseconds: 1000 * 3600 * 24 * 30,
};

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

impl FromStr for Duration {
    type Err = DurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(\d+d|\d+h|\d+m)").unwrap();

        if !regex.is_match(s) {
            return Err(DurationError::NoMatches);
        }

        let mut millis = 0;

        for timepart_capture in regex.captures_iter(s) {
            let time_piece = timepart_capture.get(0).unwrap().as_str();
            println!("{:?}", time_piece);
            let (time_amount_str, unit) = time_piece.split_at(time_piece.len() - 1);
            let time_amount: u64 = time_amount_str.parse().unwrap();
            match unit {
                "d" => {
                    millis += 24 * 60 * 60 * 1000 * time_amount;
                }
                "h" => {
                    millis += 60 * 60 * 1000 * time_amount;
                }
                "m" => {
                    millis += 60 * 1000 * time_amount;
                }
                _ => {
                    panic!("should be unreachable, check duration regex")
                }
            }
        }

        Ok(Duration::Limited {
            milliseconds: millis,
        })
    }
}

#[derive(Debug)]
pub enum DurationError {
    NoMatches,
}

impl Display for DurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationError::NoMatches => write!(f, "No matches in the valid format found!"),
        }
    }
}

impl Error for DurationError {}
