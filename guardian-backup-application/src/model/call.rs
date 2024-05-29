use serde::{Deserialize, Serialize};
use guardian_backup_domain::model::backup::backup::Backup;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Call {
    CreateBackup(Backup)
}
