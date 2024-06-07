use guardian_backup_domain::hash_service::{Hasher, PendingHashB};
use guardian_backup_domain::model::files::file_hash::FileHash;
use guardian_backup_domain::model::files::file_hash::FileHash::Blake3;

pub struct BlakeHasher();

impl Hasher for BlakeHasher {
    fn preference(&self) -> i8 {
        10
    }

    fn can_compare_hash(&self, hash: &FileHash) -> bool {
        matches!(hash, Blake3 { .. })
    }

    fn create_hash(&self) -> Box<dyn PendingHashB> {
        Box::new(PendingBlakeHash {
            digest: blake3::Hasher::new(),
        })
    }
}

pub struct PendingBlakeHash {
    digest: blake3::Hasher,
}

impl PendingHashB for PendingBlakeHash {
    fn update(&mut self, data: &[u8]) {
        self.digest.update(data);
    }

    fn finalize(&self) -> FileHash {
        let mut data = [0; 64];
        self.digest.finalize_xof().fill(data.as_mut_slice());
        Blake3 { hash: data.into() }
    }
}
