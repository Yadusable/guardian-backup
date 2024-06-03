use crate::model::files::file_hash::FileHash;
use std::fmt::Error;

pub trait BlobBuilder {
    type Error;

    async fn append_bytes(&mut self, data: &[u8]) -> Result<(), Error>;

    fn get_hash(&self) -> FileHash;
}
