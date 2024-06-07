use crate::model::backup::backup::{Backup, BackupId};
use crate::model::user_identifier::UserIdentifier;

pub trait BackupRepository {
    type Error: 'static + std::error::Error;
    async fn get_backups(
        &mut self,
        user: &UserIdentifier,
    ) -> Result<Box<dyn Iterator<Item = Backup> + '_>, Self::Error>;
    async fn get_backup_by_id(
        &mut self,
        id: &BackupId,
        user: &UserIdentifier,
    ) -> Result<Option<Backup>, Self::Error>;
    async fn update_backup(
        &mut self,
        backup: Backup,
        user: &UserIdentifier,
    ) -> Result<(), Self::Error>;
    async fn create_backup(
        &mut self,
        user: &UserIdentifier,
        backup: Backup,
    ) -> Result<(), Self::Error>;
}
