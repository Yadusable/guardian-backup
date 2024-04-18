use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct UserIdentifier(Box<str>);

impl From<Box<str>> for UserIdentifier {
    fn from(value: Box<str>) -> Self {
        UserIdentifier(value)
    }
}

impl Display for UserIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
