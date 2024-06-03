use crate::model::hash_service::{HashService, PendingHash};
use guardian_backup_domain::model::files::file_hash::FileHash;

pub struct MockHashService();

impl HashService for MockHashService {
    type PendingHash = MockPendingHash;

    fn create_hash() -> Self::PendingHash {
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
