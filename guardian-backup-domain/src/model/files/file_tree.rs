use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::files::directory_metadata::DirectoryMetadata;
use crate::model::files::file_metadata::FileMetadata;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileTreeNode {
    File {
        name: OsString,
        blob: BlobIdentifier,
        metadata: FileMetadata,
    },
    Directories {
        name: OsString,
        metadata: DirectoryMetadata,
        children: Vec<FileTreeNode>,
    },
    SymbolicLink {
        name: OsString,
        target: Box<FileTreeNode>,
    },
}

pub struct FileTreeDiff {
    pub diff_type: FileTreeDiffType,
    pub node: FileTreeNode,
    pub location: PathBuf,
}

pub enum FileTreeDiffType {
    Created,
    Updated,
    Deleted,
}
