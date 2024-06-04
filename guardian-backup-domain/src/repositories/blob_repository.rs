use crate::model::blobs::blob_fetch::BlobFetch;
use crate::model::blobs::blob_identifier::BlobIdentifier;

pub trait BlobRepository {
    type Error;

    async fn insert_blob(
        &mut self,
        id: &BlobIdentifier,
        blob: impl BlobFetch,
    ) -> Result<(), Self::Error>;
    async fn delete_blob(&mut self, id: &BlobIdentifier) -> Result<(), Self::Error>;
    async fn fetch_blob(&self, id: &BlobIdentifier) -> Result<impl BlobFetch, Self::Error>;
}
