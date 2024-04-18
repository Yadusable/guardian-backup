use std::cmp::min;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::Error;
use std::ops::Deref;
use guardian_backup_domain::model::blobs::blob_builder::BlobBuilder;
use guardian_backup_domain::model::blobs::blob_creation_hint::BlobCreationHint;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::model::files::file_hash::FileHash;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::blob_repository::BlobRepository;
use crate::model::hash_service::{HashService, PendingHash};
use crate::model::mocks::mock_hash_service::{MockHashService, MockPendingHash};

pub struct InMemoryBlobRepository {
    blobs: HashMap<BlobIdentifier, Box<[u8]>>
}

impl BlobRepository for InMemoryBlobRepository {
    type Error = Infallible;
    type Builder = InMemoryBlobBuilder<MockPendingHash>;
    type BlobFetch = InMemoryBlobFetch;

    async fn start_create_blob(&self, user: &UserIdentifier, _hint: &BlobCreationHint) -> Result<Self::Builder, Self::Error> {
        Ok(InMemoryBlobBuilder{
            user: user.clone(),
            data: vec![],
            hash: MockHashService::create_hash(),
        })
    }

    async fn finalize_blob(&mut self, builder: Self::Builder) -> Result<BlobIdentifier, Self::Error> {
        let blob_identifier = BlobIdentifier::new(builder.get_hash(), builder.user);
        self.blobs.insert(blob_identifier.clone(), builder.data.into());
        Ok(blob_identifier)
    }

    async fn delete_blob(&mut self, user: &UserIdentifier, blob: &BlobIdentifier) -> Result<(), Self::Error> {
        todo!()
    }

    fn fetch_blob(&self, user: &UserIdentifier, blob: &BlobIdentifier) -> Result<Self::BlobFetch, Self::Error> {
        todo!()
    }
}

pub struct InMemoryBlobFetch {
    data: Box<[u8]>,
    cursor: usize,
}

impl BlobFetch for InMemoryBlobFetch {
    type Error = Infallible;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let to_read = min(self.data.len() - self.cursor, buf.len());
        buf.copy_from_slice(self.data.deref().split_at(self.cursor).1.split_at(to_read).0);
        self.cursor += to_read;
        Ok(to_read)
    }

    async fn read_to_eof(self) -> Result<Box<[u8]>, Self::Error> {
        Ok(self.data)
    }
}

pub struct InMemoryBlobBuilder<H: PendingHash> {
    data: Vec<u8>,
    user: UserIdentifier,
    hash: H,
}

impl<H: PendingHash> BlobBuilder for InMemoryBlobBuilder<H> {
    type Error = Infallible;

    async fn append_bytes(&mut self, data: &[u8]) -> Result<(), Error> {
        self.data.extend_from_slice(data);
        self.hash.update(data);
        Ok(())
    }

    fn get_hash(&self) -> FileHash {
        self.hash.finalize()
    }
}