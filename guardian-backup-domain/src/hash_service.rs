use crate::model::blobs::blob_fetch::BlobFetch;
use crate::model::files::file_hash::FileHash;

pub struct HashService {
    supported_hashers: Vec<&'static dyn Hasher<PendingHash = Box<dyn PendingHash>>>,
}

impl HashService {
    pub fn preferred_hasher(&self) -> &'static dyn Hasher<PendingHash = Box<dyn PendingHash>> {
        let preferred = *self
            .supported_hashers
            .iter()
            .max_by_key(|e| e.preference())
            .unwrap();

        preferred
    }

    pub fn find_compatible_hasher(
        &self,
        hash: &FileHash,
    ) -> &'static dyn Hasher<PendingHash = Box<dyn PendingHash>> {
        *self
            .supported_hashers
            .iter()
            .find(|e| e.can_compare_hash(hash))
            .unwrap()
    }
}

pub trait Hasher {
    type PendingHash;

    fn preference(&self) -> i8;
    fn can_compare_hash(&self, hash: &FileHash) -> bool;
    fn create_hash(&self) -> Self::PendingHash;
}

pub trait PendingHash {
    fn update(&mut self, data: &[u8]);
    fn finalize(&self) -> FileHash;
}

pub trait PendingHashExt: PendingHash {
    async fn update_blob<B: BlobFetch>(&mut self, mut blob: B) -> Result<(), B::Error> {
        let mut buf = [0; 4096];

        loop {
            let read = blob.read(&mut buf).await?;
            if read == 0 {
                return Ok(());
            } else {
                self.update(&buf[..read])
            }
        }
    }
}

impl<T: PendingHash + ?Sized> PendingHashExt for T {}
