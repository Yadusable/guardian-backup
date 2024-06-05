use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;
use guardian_backup_application::file_service::FileService;
use guardian_backup_domain::hash_service::{Hasher, PendingHash, PendingHashExt};
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::model::files::directory_metadata::DirectoryMetadata;
use guardian_backup_domain::model::files::file_metadata::FileMetadata;
use guardian_backup_domain::model::files::file_tree::{
    FileTreeDiff, FileTreeDiffType, FileTreeNode,
};
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::iter::{empty, once};
use std::path::{Path, PathBuf};
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

    async fn compare_to_file_tree(
        path: &Path,
        file_tree_node: &FileTreeNode,
    ) -> Result<Box<dyn Iterator<Item = FileTreeDiff>>, Self::Error> {
        match file_tree_node {
            FileTreeNode::File {
                name,
                blob,
                metadata,
            } => {
                let current_fs_path = path.join(name);
                if !tokio::fs::try_exists(current_fs_path.as_path())
                    .await
                    .unwrap_or(false)
                {
                    return Ok(Box::new(once(FileTreeDiff {
                        diff_type: FileTreeDiffType::Created,
                        node: file_tree_node.clone(),
                        location: path.into(),
                    })));
                }
                let current_fs_file = tokio::fs::metadata(current_fs_path.as_path()).await?;
                if current_fs_file
                    .modified()?
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
                    != metadata.last_modified
                {
                    return Ok(Box::new(once(FileTreeDiff {
                        diff_type: FileTreeDiffType::Updated,
                        node: file_tree_node.clone(),
                        location: path.into(),
                    })));
                } else {
                    return Ok(Box::new(empty()));
                }
            }
            FileTreeNode::Directory {
                name,
                metadata,
                children,
            } => {
                let dir_path = path.join(name);

                if !tokio::fs::try_exists(dir_path.as_path()).await? {
                    return Ok(Box::new(once(FileTreeDiff {
                        diff_type: FileTreeDiffType::Created,
                        node: file_tree_node.clone(),
                        location: path.into(),
                    })));
                }

                let mut res = vec![];
                for child in children {
                    res.push(
                        Box::pin(Self::compare_to_file_tree(dir_path.as_path(), child)).await?,
                    );
                }
                let res = res.into_iter().map(|e| e).flatten();

                return Ok(Box::new(res));
            }
            FileTreeNode::SymbolicLink { .. } => {
                todo!()
            }
        }
    }
}
