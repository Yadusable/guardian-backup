use crate::model::timestamp::Timestamp;

#[derive(Debug)]
pub struct BlobMetadata {
    expiration_time: Timestamp,
    //TODO Encryption and Compression
}

impl BlobMetadata {
    pub fn new(expiration_time: Timestamp) -> Self {
        Self { expiration_time }
    }

    pub fn expiration_time(&self) -> Timestamp {
        self.expiration_time
    }
}
