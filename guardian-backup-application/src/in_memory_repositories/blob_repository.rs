use crate::in_memory_repositories::blob_repository::BlobRepositoryError::ReadBlobError;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::repositories::blob_repository::BlobRepository;
use std::cmp::min;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;

pub struct InMemoryBlobRepository {
    blobs: HashMap<BlobIdentifier, Arc<[u8]>>,
}

impl BlobRepository for InMemoryBlobRepository {
    type Error = BlobRepositoryError;

    async fn insert_blob(
        &mut self,
        id: BlobIdentifier,
        mut blob: impl BlobFetch,
    ) -> Result<(), Self::Error> {
        if self.blobs.contains_key(&id) {
            return Ok(());
        }

        let data = blob
            .read_to_eof()
            .await
            .map_err(|e| ReadBlobError(e.into()))?;
        self.blobs.insert(id, data.into());
        Ok(())
    }

    async fn delete_blob(&mut self, blob: &BlobIdentifier) -> Result<(), Self::Error> {
        self.blobs
            .remove(blob)
            .ok_or(BlobRepositoryError::BlobNotFound)
            .map(|_| ())
    }

    async fn fetch_blob(&self, blob: &BlobIdentifier) -> Result<impl BlobFetch, Self::Error> {
        Ok(InMemoryBlobFetch {
            data: self
                .blobs
                .get(blob)
                .ok_or(BlobRepositoryError::BlobNotFound)?
                .clone(),
            cursor: 0,
        })
    }
}

#[derive(Debug)]
pub enum BlobRepositoryError {
    BlobNotFound,
    ReadBlobError(Box<dyn std::error::Error>),
}

impl Display for BlobRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlobRepositoryError::BlobNotFound => write!(f, "BlobNotFound"),
            ReadBlobError(e) => write!(f, "BlobReadError{e}"),
        }
    }
}

impl std::error::Error for BlobRepositoryError {}

#[derive(Debug)]
pub struct InMemoryBlobFetch {
    data: Arc<[u8]>,
    cursor: usize,
}

impl InMemoryBlobFetch {
    pub fn new(data: Arc<[u8]>) -> Self {
        Self { data, cursor: 0 }
    }
}

impl BlobFetch for InMemoryBlobFetch {
    type Error = Infallible;

    fn remaining_len(&self) -> u64 {
        (self.data.len() - self.cursor) as u64
    }

    fn total_len(&self) -> u64 {
        self.data.len() as u64
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let to_read = min(self.data.len() - self.cursor, buf.len());
        buf.split_at_mut(to_read).0.copy_from_slice(
            self.data
                .deref()
                .split_at(self.cursor)
                .1
                .split_at(to_read)
                .0,
        );
        self.cursor += to_read;
        Ok(to_read)
    }
}
