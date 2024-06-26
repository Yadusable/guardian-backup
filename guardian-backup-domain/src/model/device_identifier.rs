use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIdentifier {
    identifier: Box<str>,
}

impl Default for DeviceIdentifier {
    fn default() -> Self {
        Self {
            identifier: "DefaultDevice".into(),
        }
    }
}
