use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FileHash {
    Blake3 {
        hash: Box<[u8]>, //TODO make it a 64 element array
    },
}
