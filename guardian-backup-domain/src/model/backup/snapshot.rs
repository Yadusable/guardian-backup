use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::timestamp::Timestamp;

#[derive(Debug)]
pub struct Snapshot {
    timestamp: Timestamp,
    expiration_time: Timestamp,
    file_tree_blob: BlobIdentifier,
}

impl Snapshot {
    pub fn new(
        timestamp: Timestamp,
        expiration_time: Timestamp,
        file_tree_blob: BlobIdentifier,
    ) -> Self {
        Self {
            timestamp,
            expiration_time,
            file_tree_blob,
        }
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
    pub fn expiration_time(&self) -> Timestamp {
        self.expiration_time
    }
    pub fn file_tree_blob(&self) -> &BlobIdentifier {
        &self.file_tree_blob
    }
}
