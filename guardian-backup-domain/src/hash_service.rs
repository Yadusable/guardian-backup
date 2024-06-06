use crate::model::blobs::blob_fetch::BlobFetch;
use crate::model::files::file_hash::FileHash;

pub struct HashService {}

impl HashService {
    pub fn preferred_hasher(&self) -> Box<dyn Hasher<PendingHashA = dyn PendingHashB>> {
        todo!()
    }
}

pub trait Hasher {
    type PendingHashA: PendingHashB + ?Sized;

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

// pub async fn update_hash_with_blob<B: BlobFetch>(
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
