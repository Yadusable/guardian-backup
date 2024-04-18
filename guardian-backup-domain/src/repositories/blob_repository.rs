use crate::model::blobs::blob_builder::BlobBuilder;
use crate::model::blobs::blob_creation_hint::BlobCreationHint;
use crate::model::blobs::blob_fetch::BlobFetch;
use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::user_identifier::UserIdentifier;

pub trait BlobRepository {
    type Error;
    type Builder: BlobBuilder;
    type BlobFetch: BlobFetch;

    async fn start_create_blob(&self, user: &UserIdentifier, hint: &BlobCreationHint) -> Result<Self::Builder, Self::Error>;
    async fn finalize_blob(&mut self, builder: Self::Builder) -> Result<BlobIdentifier, Self::Error>;

    async fn delete_blob(&mut self, blob: &BlobIdentifier) -> Result<(), Self::Error>;
    fn fetch_blob(&self, blob: &BlobIdentifier) -> Result<Self::BlobFetch, Self::Error>;
}
