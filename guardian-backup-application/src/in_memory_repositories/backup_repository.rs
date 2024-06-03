use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::Infallible;

pub struct InMemoryBackupRepository {
    backups: HashMap<UserIdentifier, Vec<Backup>>,
}

impl BackupRepository for InMemoryBackupRepository {
    type Error = Infallible;

    async fn get_backups(&self, user: &UserIdentifier) -> Result<Cow<[Backup]>, Self::Error> {
        Ok(match self.backups.get(user) {
            None => Cow::Owned(vec![]),
            Some(res) => Cow::Borrowed(res),
        })
    }

    async fn create_backup(
        &mut self,
        user: &UserIdentifier,
        backup: Backup,
    ) -> Result<(), Self::Error> {
        if let Some(backups) = self.backups.get_mut(user) {
            backups.push(backup);
        } else {
            let mut backups = Vec::new();
            backups.push(backup);
            self.backups.insert(user.clone(), backups);
        }

        Ok(())
    }
}
