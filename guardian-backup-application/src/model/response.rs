use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Response {
    Successful,
    Error(Box<str>),
    BackupList(Vec<Backup>),
    BlobCreated(BlobIdentifier),
}
