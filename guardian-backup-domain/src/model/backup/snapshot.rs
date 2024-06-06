use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Snapshot {
    timestamp: Timestamp,
    expiration_time: Option<Timestamp>,
    file_tree_blob: BlobIdentifier,
    associated_blobs: Vec<BlobIdentifier>,
}

impl Snapshot {
    pub fn new(
        timestamp: Timestamp,
        expiration_time: Option<Timestamp>,
        file_tree_blob: BlobIdentifier,
        associated_blobs: Vec<BlobIdentifier>,
    ) -> Self {
        Self {
            timestamp,
            expiration_time,
            file_tree_blob,
            associated_blobs,
        }
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
    pub fn expiration_time(&self) -> Timestamp {
        self.expiration_time.unwrap()
    }
    pub fn file_tree_blob(&self) -> &BlobIdentifier {
        &self.file_tree_blob
    }
}
