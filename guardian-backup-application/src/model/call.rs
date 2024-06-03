use guardian_backup_domain::model::backup::backup::Backup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Call {
    CreateBackup(Backup),
}
