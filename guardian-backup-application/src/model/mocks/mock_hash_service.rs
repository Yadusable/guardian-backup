use guardian_backup_domain::hash_service::{Hasher, PendingHash};
use guardian_backup_domain::model::files::file_hash::FileHash;

pub struct MockHasher();

impl Hasher for MockHasher {
    type PendingHash = MockPendingHash;

    fn preference(&self) -> i8 {
        0
    }

    fn can_compare_hash(&self, hash: &FileHash) -> bool {
        hash == &FileHash::Mock
    }

    fn create_hash(&self) -> Self::PendingHash {
        MockPendingHash()
    }
}

pub struct MockPendingHash();

impl PendingHash for MockPendingHash {
    fn update(&mut self, _data: &[u8]) {}

    fn finalize(&self) -> FileHash {
        FileHash::Mock
    }
}
