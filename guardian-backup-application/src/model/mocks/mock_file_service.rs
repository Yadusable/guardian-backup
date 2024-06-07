use crate::file_service::{File, FileService};
use crate::in_memory_repositories::blob_repository::InMemoryBlobFetch;
use guardian_backup_domain::hash_service::{Hasher, PendingHashB, PendingHashExt};
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::model::files::directory_metadata::DirectoryMetadata;
use guardian_backup_domain::model::files::file_hash::FileHash;
use guardian_backup_domain::model::files::file_metadata::FileMetadata;
use guardian_backup_domain::model::files::file_tree::FileTreeNode;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub struct MockFileService {}

impl FileService for MockFileService {
    type File = MockFile;
    type Error = Infallible;

    async fn get_file(path: &Path) -> Result<Self::File, Self::Error> {
        Ok(MockFile {
            hash: FileHash::Mock,
            size: 43,
            last_modified: 223355779,
        })
    }

    async fn generate_file_tree(
        path: &Path,
        hasher: &dyn Hasher,
        user: &UserIdentifier,
    ) -> Result<FileTreeNode, Self::Error> {
        Ok(FileTreeNode::File {
            name: Default::default(),
            blob: BlobIdentifier::new(FileHash::Mock, UserIdentifier::new("Mock".into())),
            metadata: FileMetadata {
                file_size: 42,
                last_modified: 123456789,
            },
        })
    }

    async fn delete_file(path: &Path) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn delete_dir_all(path: &Path) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn write_file(
        path: &Path,
        file_meta: &FileMetadata,
        blob: impl BlobFetch,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct MockFile {
    hash: FileHash,
    size: u64,
    last_modified: u64,
}

impl File for MockFile {
    type Error = Infallible;

    async fn get_hash<H: Hasher>(&self, hasher: H) -> Result<FileHash, Self::Error> {
        Ok(self.hash.clone())
    }

    async fn get_size(&self) -> Result<u64, Self::Error> {
        Ok(self.size)
    }

    async fn get_last_modified(&self) -> Result<u64, Self::Error> {
        Ok(self.last_modified)
    }

    async fn get_as_blob(&self) -> Result<impl BlobFetch, Self::Error> {
        Ok(InMemoryBlobFetch::new([0xfe; 64].into()))
    }
}
