use std::borrow::Cow;
use crate::model::backup::backup::Backup;
use crate::model::user_identifier::UserIdentifier;

pub trait BackupRepository {
    type Error: 'static + std::error::Error;
    async fn get_backups(&self, user: &UserIdentifier) -> Result<Cow<[Backup]>, Self::Error>;
    async fn create_backup(&mut self, user: &UserIdentifier, backup: Backup) -> Result<(), Self::Error>;
}
