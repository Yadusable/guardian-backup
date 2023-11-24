use crate::model::blobs::blob_builder::BlobBuilder;
use crate::model::blobs::blob_fetch::BlobFetch;
use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::error::{AsyncResult, Result};
use crate::model::user_identifier::UserIdentifier;

pub trait BlobRepository<B: BlobBuilder + Sized> {
    fn start_create_blob(user: &UserIdentifier) -> Result<B>;
    fn finalize_blob(builder: &B) -> AsyncResult<BlobIdentifier>;

    fn delete_blob(user: &UserIdentifier, blob: &BlobIdentifier) -> Result<()>;
    fn fetch_blob(user: &UserIdentifier, blob: &BlobIdentifier) -> Result<Box<dyn BlobFetch>>;
}
