use guardian_backup_domain::hash_service::{Hasher, PendingHashB};
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::files::file_hash::FileHash;
use guardian_backup_domain::model::files::file_metadata::FileMetadata;
use guardian_backup_domain::model::files::file_tree::{FileTreeDiff, FileTreeNode};
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::error::Error;
use std::path::Path;

pub trait FileService {
    type File: File;
    type Error: Error + 'static;

    async fn get_file(path: &Path) -> Result<Self::File, Self::Error>;
    async fn generate_file_tree(
        path: &Path,
        hasher: &dyn Hasher,
        user: &UserIdentifier,
    ) -> Result<FileTreeNode, Self::Error>;

    async fn delete_file(path: &Path) -> Result<(), Self::Error>;
    async fn delete_dir_all(path: &Path) -> Result<(), Self::Error>;
    async fn write_file(
        path: &Path,
        file_meta: &FileMetadata,
        blob: impl BlobFetch,
    ) -> Result<(), Self::Error>;
    async fn create_dir(path: &Path) -> Result<(), Self::Error>;
}

pub trait File {
    type Error: Error + 'static;

    async fn get_hash<H: Hasher>(&self, hasher: H) -> Result<FileHash, Self::Error>;
    async fn get_size(&self) -> Result<u64, Self::Error>;
    async fn get_last_modified(&self) -> Result<u64, Self::Error>;
    async fn get_as_blob(&self) -> Result<impl BlobFetch, Self::Error>;
}
