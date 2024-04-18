use serde::{Deserialize, Serialize};
use crate::model::files::file_hash::FileHash;
use crate::model::user_identifier::UserIdentifier;

#[derive(Eq, PartialEq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct BlobIdentifier {
    hash: FileHash,
    user: UserIdentifier,
}

impl BlobIdentifier {
    pub fn new(hash: FileHash, user: UserIdentifier) -> Self {
        Self { hash, user }
    }
    pub fn hash(&self) -> &FileHash {
        &self.hash
    }
    pub fn user(&self) -> &UserIdentifier {
        &self.user
    }
}
