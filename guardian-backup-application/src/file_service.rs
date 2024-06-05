use guardian_backup_domain::hash_service::Hasher;
use guardian_backup_domain::model::files::file_hash::FileHash;
use guardian_backup_domain::model::files::file_tree::{FileTreeDiff, FileTreeNode};
use std::error::Error;
use std::path::Path;

pub trait FileService {
    type File;
    type Error: Error + 'static;

    async fn get_file(path: &Path) -> Result<Self::File, Self::Error>;

    async fn compare_to_file_tree(
        path: &Path,
        file_tree_node: &FileTreeNode,
    ) -> Result<Box<dyn Iterator<Item = FileTreeDiff>>, Self::Error>;
}

pub trait File {
    type Error: Error + 'static;

    async fn get_hash<H: Hasher>(&self) -> Result<FileHash, Self::Error>;
    async fn get_size(&self) -> Result<u64, Self::Error>;
    async fn get_last_modified(&self) -> Result<u64, Self::Error>;
}
