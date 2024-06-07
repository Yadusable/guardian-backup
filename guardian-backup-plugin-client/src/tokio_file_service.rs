use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;
use crate::tokio_file::TokioFile;
use crate::tokio_file_service::TokioFileServiceError::BlobRead;
use guardian_backup_application::file_service::FileService;
use guardian_backup_domain::hash_service::{Hasher, PendingHashB, PendingHashExt};
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::model::files::directory_metadata::DirectoryMetadata;
use guardian_backup_domain::model::files::file_metadata::FileMetadata;
use guardian_backup_domain::model::files::file_tree::FileTreeNode;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;

pub struct TokioFileService {}

impl FileService for TokioFileService {
    type File = TokioFile;
    type Error = TokioFileServiceError;

    async fn get_file(path: &Path) -> Result<Self::File, Self::Error> {
        Ok(TokioFile::new(path.into()))
    }

    async fn generate_file_tree(
        path: &Path,
        hasher: &dyn Hasher,
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

    async fn delete_file(path: &Path) -> Result<(), Self::Error> {
        Ok(tokio::fs::remove_file(path).await?)
    }

    async fn delete_dir_all(path: &Path) -> Result<(), Self::Error> {
        Ok(tokio::fs::remove_dir_all(path).await?)
    }

    async fn write_file(
        path: &Path,
        file_meta: &FileMetadata,
        mut blob: impl BlobFetch,
    ) -> Result<(), Self::Error> {
        let mut file = tokio::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .await?;

        let mut chunk = [0; 4096];

        loop {
            let read = blob
                .read(&mut chunk)
                .await
                .map_err(|e| BlobRead(e.into()))?;
            if read == 0 {
                break;
            }

            file.write_all(&chunk[..read]).await?;
        }

        let file_meta = file_meta.clone();
        let file = file.into_std().await;
        tokio::task::spawn_blocking(move || {
            file.set_times(
                std::fs::FileTimes::new()
                    .set_modified(UNIX_EPOCH.add(Duration::from_millis(file_meta.last_modified))),
            )?;
            Ok::<(), std::io::Error>(())
        })
        .await
        .unwrap()?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum TokioFileServiceError {
    Tokio(tokio::io::Error),
    BlobRead(Box<dyn Error>),
}

impl Display for TokioFileServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokioFileServiceError::Tokio(inner) => write!(f, "Tokio({inner})"),
            TokioFileServiceError::BlobRead(inner) => write!(f, "BlobRead({inner})"),
        }
    }
}

impl Error for TokioFileServiceError {}

impl From<tokio::io::Error> for TokioFileServiceError {
    fn from(value: tokio::io::Error) -> Self {
        Self::Tokio(value)
    }
}
