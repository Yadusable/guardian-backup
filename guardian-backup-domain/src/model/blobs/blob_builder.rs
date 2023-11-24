use crate::model::error::AsyncResult;
use crate::model::files::file_hash::FileHash;

pub trait BlobBuilder {
    fn append_bytes(&mut self, data: &[u8]) -> AsyncResult<()>;

    fn get_hash(&self) -> AsyncResult<FileHash>;
}
