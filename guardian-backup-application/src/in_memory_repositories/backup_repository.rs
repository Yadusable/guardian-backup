use guardian_backup_domain::model::backup::backup::{Backup, BackupId};
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;
use std::collections::HashMap;
use std::convert::Infallible;
use std::iter::empty;

pub struct InMemoryBackupRepository {
    backups: HashMap<UserIdentifier, HashMap<BackupId, Backup>>,
}

impl InMemoryBackupRepository {
    pub fn new() -> Self {
        InMemoryBackupRepository {
            backups: HashMap::new(),
        }
    }
}

impl BackupRepository for InMemoryBackupRepository {
    type Error = Infallible;

    async fn get_backups(
        &mut self,
        user: &UserIdentifier,
    ) -> Result<Box<dyn Iterator<Item = Backup> + '_>, Self::Error> {
        Ok(match self.backups.get(user) {
            None => Box::new(empty()),
            Some(res) => Box::new(res.values().cloned()),
        })
    }

    async fn get_backup_by_id(
        &mut self,
        id: &BackupId,
        user: &UserIdentifier,
    ) -> Result<Option<Backup>, Self::Error> {
        Ok(self
            .backups
            .get(user)
            .and_then(|backups| backups.get(id).cloned()))
    }

    async fn update_backup(
        &mut self,
        backup: Backup,
        user: &UserIdentifier,
    ) -> Result<(), Self::Error> {
        self.backups
            .get_mut(user)
            .and_then(|e| e.insert(backup.id().clone(), backup));

        Ok(())
    }

    async fn create_backup(
        &mut self,
        user: &UserIdentifier,
        backup: Backup,
    ) -> Result<(), Self::Error> {
        if let Some(backups) = self.backups.get_mut(user) {
            backups.insert(backup.id().clone(), backup);
        } else {
            let mut backups = HashMap::new();
            backups.insert(backup.id().clone(), backup);
            self.backups.insert(user.clone(), backups);
        }

        Ok(())
    }
}
