use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_size: u64,
    pub last_modified: u64,
}

impl FileMetadata {
    pub fn last_modified(&self) -> u64 {
        self.last_modified
    }
    pub fn file_size(&self) -> u64 {
        self.file_size
    }
}
