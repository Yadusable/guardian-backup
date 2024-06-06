use guardian_backup_domain::hash_service::{Hasher, PendingHashB};
use guardian_backup_domain::model::files::file_hash::FileHash;

pub struct MockHashService();

impl Hasher for MockHashService {
    type PendingHashA = MockPendingHash;

    fn can_compare_hash(&self, hash: &FileHash) -> bool {
        hash == &FileHash::Mock
    }

    fn create_hash(&self) -> Box<Self::PendingHashA> {
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
