use crate::model::blobs::blob_fetch::BlobFetch;
use crate::model::files::file_hash::FileHash;

pub struct HashService {
    supported_hashers: Vec<&'static dyn Hasher<PendingHashA = dyn PendingHashB>>,
}

// pub async fn update_hash_with_blob<B: BlobFetch>(

impl HashService {
    pub fn preferred_hasher(&self) -> &'static dyn Hasher<PendingHashA = dyn PendingHashB> {
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
    ) -> &'static dyn Hasher<PendingHashA = dyn PendingHashB> {
        *self
            .supported_hashers
            .iter()
            .find(|e| e.can_compare_hash(hash))
            .unwrap()
    }
}

pub trait Hasher {
    type PendingHashA: PendingHashB + ?Sized;

    fn preference(&self) -> i8;
    fn can_compare_hash(&self, hash: &FileHash) -> bool;
    fn create_hash(&self) -> Box<Self::PendingHashA>;
}

pub trait PendingHashB {
    fn update(&mut self, data: &[u8]);
    fn finalize(&self) -> FileHash;
}

pub trait PendingHashExt: PendingHashB {
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

impl<T: PendingHashB + ?Sized> PendingHashExt for T {}
//     hash: &mut dyn PendingHashB,
//     mut blob: B,
// ) -> Result<(), B::Error> {
//     let mut buf = [0; 4096];
//
//     loop {
//         let read = blob.read(&mut buf).await?;
//         if read == 0 {
//             return Ok(());
//         } else {
//             hash.update(&buf[..read])
//         }
//     }
// }
