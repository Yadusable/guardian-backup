use guardian_backup_domain::hash_service::{Hasher, PendingHashB};
use guardian_backup_domain::model::files::file_hash::FileHash;

pub struct MockHasher();

pub const MOCK_HASHER: MockHasher = MockHasher();

impl Hasher for MockHasher {
    fn preference(&self) -> i8 {
        0
    }

    fn can_compare_hash(&self, hash: &FileHash) -> bool {
        hash == &FileHash::Mock
    }

    fn create_hash(&self) -> Box<dyn PendingHashB> {
        Box::new(MockPendingHash())
    }
}

pub struct MockPendingHash();

impl PendingHashB for MockPendingHash {
    fn update(&mut self, _data: &[u8]) {}

    fn finalize(&self) -> FileHash {
        FileHash::Mock
    }
}
