use crate::model::call::Call;
use crate::model::connection_interface::ConnectionClientInterface;
use crate::model::connection_interface::IncomingResponse;
use crate::model::response::Response;
use guardian_backup_domain::model::backup::backup::{Backup, BackupId};
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;

pub struct RemoteBackupRepository<C: ConnectionClientInterface> {
    connection_interface: C,
}

impl<C: ConnectionClientInterface> RemoteBackupRepository<C> {
    pub fn new(connection_interface: C) -> Self {
        Self {
            connection_interface,
        }
    }
}

impl<C: ConnectionClientInterface> BackupRepository for RemoteBackupRepository<C> {
    type Error = C::Error;

    async fn get_backups(
        &mut self,
        _user: &UserIdentifier, //TODO user handling
    ) -> Result<Box<dyn Iterator<Item = Backup> + '_>, Self::Error> {
        let res = self
            .connection_interface
            .send_request(Call::GetBackups)
            .await?;

        let res = res.into_inner();

        if let Response::BackupList(backups) = res {
            Ok(Box::new(backups.into_vec().into_iter()))
        } else {
            todo!()
        }
    }

    async fn get_backup_by_id(
        &mut self,
        id: &BackupId,
        user: &UserIdentifier,
    ) -> Result<Option<Backup>, Self::Error> {
        self.get_backups(user)
            .await
            .map(|mut e| e.find(|e| e.id() == id))
    }

    async fn update_backup(
        &mut self,
        backup: Backup,
        user: &UserIdentifier,
    ) -> Result<(), Self::Error> {
        let call = Call::PatchBackup(backup);
        //TODO User check
        let res = self.connection_interface.send_request(call).await?;

        if let &Response::Successful = res.inner() {
            Ok(())
        } else {
            todo!()
        }
    }

    async fn create_backup(
        &mut self,
        user: &UserIdentifier,
        backup: Backup,
    ) -> Result<(), Self::Error> {
        let call = Call::CreateBackup(backup);
        let res = self.connection_interface.send_request(call).await?;

        if &Response::Successful == res.inner() {
            Ok(())
        } else {
            todo!()
        }
    }
}
