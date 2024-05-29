use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct DeviceIdentifier {
    identifier: Box<str>,
}

impl Default for DeviceIdentifier {
    fn default() -> Self {
        Self{
            identifier: "DefaultDevice".into()
        }
    }
}
