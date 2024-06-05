use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;
use guardian_backup_application::file_service::FileService;
use guardian_backup_domain::hash_service::{Hasher, PendingHash, PendingHashExt};
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::model::files::directory_metadata::DirectoryMetadata;
use guardian_backup_domain::model::files::file_metadata::FileMetadata;
use guardian_backup_domain::model::files::file_tree::FileTreeNode;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub struct TokioFileService {}

impl FileService for TokioFileService {
    type File = tokio::fs::File;
    type Error = tokio::io::Error;

    async fn get_file(path: &Path) -> Result<Self::File, Self::Error> {
        tokio::fs::File::open(path).await
    }

    async fn generate_file_tree(
        path: &Path,
        hasher: &dyn Hasher<PendingHash = Box<dyn PendingHash>>,
        user: &UserIdentifier,
    ) -> Result<FileTreeNode, Self::Error> {
        let metadata = tokio::fs::metadata(path).await?;

        if metadata.is_file() {
            let mut hash = hasher.create_hash();
            let file = tokio::fs::File::open(path).await?;
            hash.update_blob(TokioBlobFetch::new(file, metadata.len()))
                .await?;
            let hash = hash.finalize();

            return Ok(FileTreeNode::File {
                name: path.file_name().unwrap().into(),
                blob: BlobIdentifier::new(hash, user.clone()),
                metadata: FileMetadata {
                    file_size: metadata.len(),
                    last_modified: metadata
                        .modified()?
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                },
            });
        } else if metadata.is_dir() {
            let mut children = vec![];

            let mut dir = tokio::fs::read_dir(path).await?;
            while let Some(child) = dir.next_entry().await? {
                children.push(
                    Box::pin(Self::generate_file_tree(
                        child.path().as_path(),
                        hasher,
                        user,
                    ))
                    .await?,
                )
            }

            return Ok(FileTreeNode::Directory {
                name: path.file_name().unwrap().into(),
                metadata: DirectoryMetadata {},
                children,
            });
        }

        panic!("FS entries are always either files or directories (symlinks are traversed)")
    }
}
