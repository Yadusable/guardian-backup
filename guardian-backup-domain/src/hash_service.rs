use crate::model::files::file_hash::FileHash;

pub struct HashService {}

pub trait Hasher {
    type PendingHash;

    fn can_compare_hash(&self, hash: &FileHash) -> bool;
    fn create_hash(&self) -> Self::PendingHash;
}

pub trait PendingHash {
    fn update(&mut self, data: &[u8]);

    fn finalize(&self) -> FileHash;
}
