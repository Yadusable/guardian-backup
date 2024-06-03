use guardian_backup_domain::model::files::file_hash::FileHash;

pub trait HashService {
    type PendingHash;

    fn create_hash() -> Self::PendingHash;
}

pub trait PendingHash {
    fn update(&mut self, data: &[u8]);

    fn finalize(&self) -> FileHash;
}
