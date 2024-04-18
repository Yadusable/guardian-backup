use std::borrow::Cow;
use crate::model::backup::backup::Backup;
use crate::model::user_identifier::UserIdentifier;

pub trait BackupRepository {
    type Error;
    async fn get_backups(&self, user: &UserIdentifier) -> Result<Cow<[Backup]>, Self::Error>;
}
