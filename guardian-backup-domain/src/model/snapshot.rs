use crate::model::file_tree::FileTreeNode;
use crate::model::timestamp::Timestamp;

pub struct Snapshot {
    timestamp: Timestamp,
    file_tree: FileTreeNode
}