use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::files::directory_metadata::DirectoryMetadata;
use crate::model::files::file_metadata::FileMetadata;
use std::ffi::OsString;

#[derive(Debug)]
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
