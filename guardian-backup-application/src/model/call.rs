use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Call {
    CreateBackup(Backup),
    GetBackups,
    PatchBackup,
    CreateBlob,
    GetBlob(BlobIdentifier),
}
