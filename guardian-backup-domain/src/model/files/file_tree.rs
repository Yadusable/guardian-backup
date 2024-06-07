use crate::model::blobs::blob_identifier::BlobIdentifier;
use crate::model::files::directory_metadata::DirectoryMetadata;
use crate::model::files::file_metadata::FileMetadata;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::iter::{empty, once};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileTreeNode {
    File {
        name: OsString,
        blob: BlobIdentifier,
        metadata: FileMetadata,
    },
    Directory {
        name: OsString,
        metadata: DirectoryMetadata,
        children: Vec<FileTreeNode>,
    },
    SymbolicLink {
        name: OsString,
        target: Box<FileTreeNode>,
    },
}

impl FileTreeNode {
    pub fn name(&self) -> &OsString {
        match self {
            FileTreeNode::File { name, .. } => name,
            FileTreeNode::Directory { name, .. } => name,
            FileTreeNode::SymbolicLink { name, .. } => name,
        }
    }

    pub fn diff_to<'a>(
        &'a self,
        other: &'a FileTreeNode,
        path: Box<Path>,
    ) -> Box<dyn Iterator<Item = FileTreeDiff> + 'a> {
        match self {
            FileTreeNode::File {
                name,
                blob,
                metadata,
            } => {
                if let FileTreeNode::File {
                    name: o_name,
                    blob: o_blob,
                    metadata: o_metadata,
                } = other
                {
                    if metadata.last_modified != o_metadata.last_modified {
                        return Box::new(once(FileTreeDiff {
                            diff_type: FileTreeDiffType::Updated,
                            node: self.clone(),
                            location: path.into(),
                        }));
                    } else {
                        return Box::new(empty());
                    }
                } else {
                    return Box::new(once(FileTreeDiff {
                        diff_type: FileTreeDiffType::ChangedType,
                        node: self.clone(),
                        location: path.into(),
                    }));
                }
            }
            FileTreeNode::Directory {
                name,
                metadata,
                children,
            } => {
                if let FileTreeNode::Directory {
                    name: o_name,
                    metadata: o_metadata,
                    children: o_children,
                } = other
                {
                    let new = children
                        .iter()
                        .filter(|e| !o_children.iter().any(|o| e.name() == o.name()));
                    let old = children.iter().filter_map(|e| {
                        o_children
                            .iter()
                            .find(|o| e.name() == o.name())
                            .map(|o| (e, o))
                    });
                    let gone = o_children
                        .iter()
                        .filter(|o| !children.iter().any(|e| e.name() == o.name()));

                    let path_c = path.clone();
                    let new = new.map(move |e| FileTreeDiff {
                        diff_type: FileTreeDiffType::Created,
                        node: e.clone(),
                        location: path_c.clone(),
                    });
                    let path_c = path.clone();
                    let old =
                        old.flat_map(move |(e, o)| e.diff_to(o, path_c.join(e.name()).into()));
                    let path_c = path.clone();
                    let gone = gone.map(move |o| FileTreeDiff {
                        diff_type: FileTreeDiffType::Deleted,
                        node: o.clone(),
                        location: path_c.clone(),
                    });

                    return Box::new(gone.chain(old).chain(new));
                } else {
                    return Box::new(once(FileTreeDiff {
                        diff_type: FileTreeDiffType::ChangedType,
                        node: self.clone(),
                        location: path.into(),
                    }));
                }
            }
            FileTreeNode::SymbolicLink { .. } => {
                unimplemented!()
            }
        }
    }

    pub fn iter(&self, path: PathBuf) -> Box<dyn Iterator<Item = (PathBuf, &FileTreeNode)> + '_> {
        match self {
            FileTreeNode::File { name, .. } => Box::new(once((path, self))),
            FileTreeNode::Directory { children, name, .. } => {
                let dirpath = path.join(name);

                Box::new(
                    once((path.clone(), self))
                        .chain(children.iter().flat_map(move |e| e.iter(dirpath.clone()))),
                )
            }
            FileTreeNode::SymbolicLink { .. } => {
                unimplemented!()
            }
        }
    }
}

pub struct FileTreeDiff {
    pub diff_type: FileTreeDiffType,
    pub node: FileTreeNode,
    pub location: Box<Path>,
}

pub enum FileTreeDiffType {
    Created,
    Updated,
    Deleted,
    ChangedType,
}
