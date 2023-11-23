use std::ffi::OsString;
use crate::model::blob::Blob;
use crate::model::directory_metadata::DirectoryMetadata;
use crate::model::file_metadata::FileMetadata;

pub enum FileTreeNode {
    File {
        name: OsString,
        blob: Blob,
        metadata: FileMetadata,
    },
    Directories {
        name: OsString,
        metadata: DirectoryMetadata,
        children: Vec<FileTreeNode>
    },
    SymbolicLink {
        name: OsString,
        target: Box<FileTreeNode>,
    }
}
