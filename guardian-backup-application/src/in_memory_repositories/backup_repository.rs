use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Infallible;
use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;

pub struct InMemoryBackupRepository {
    backups: HashMap<UserIdentifier, Vec<Backup>>,
}

impl BackupRepository for InMemoryBackupRepository {
    type Error = Infallible;

    async fn get_backups(&self, user: &UserIdentifier) -> Result<Cow<[Backup]>, Self::Error> {
        Ok(match self.backups.get(user) {
            None => Cow::Owned(vec![]),
            Some(res) => Cow::Borrowed(res)
        })
    }
}