use crate::model::call::Call;
use crate::model::connection_interface::{IncomingCall, UnhandledIncomingCall};
use crate::model::response::Response;
use crate::server_service::ServerServiceError::{
    BackupIdNotFound, BackupRepositoryError, ResponseError,
};
use guardian_backup_domain::model::backup::backup::BackupId;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;
use std::error::Error;

pub trait ServerService {
    type Error;

    async fn handle_incoming_request(
        &mut self,
        call: impl UnhandledIncomingCall,
    ) -> Result<(), Self::Error>;
}

pub struct MainServerService<B: BackupRepository> {
    backup_repository: B,
}

impl<B: BackupRepository> ServerService for MainServerService<B> {
    type Error = ServerServiceError;

    async fn handle_incoming_request(
        &mut self,
        call: impl UnhandledIncomingCall,
    ) -> Result<(), Self::Error> {
        let (call_variant, call) = call.into_inner();
        let user = call.user();

        match call_variant {
            Call::CreateBackup(backup) => {
                self.backup_repository
                    .create_backup(user, backup)
                    .await
                    .map_err(|err| BackupRepositoryError(err.into()))?;
                call.answer(Response::Successful)
                    .await
                    .map_err(|err| ResponseError(err.into()))?;
            }

            Call::GetBackups => {
                let backups = self
                    .backup_repository
                    .get_backups(call.user())
                    .await
                    .map_err(|e| BackupRepositoryError(e.into()))?;
                call.answer(Response::BackupList(backups.cloned().collect()))
                    .await
                    .map_err(|e| ResponseError(e.into()))?;
            }

            Call::PatchBackup(mut backup) => {
                let origin = self
                    .backup_repository
                    .take_backup_by_id(backup.id(), call.user())
                    .await
                    .map_err(|e| BackupRepositoryError(e.into()))?
                    .ok_or_else(|| BackupIdNotFound(backup.id().clone()))?;

                backup.merge_snapshots(origin.into_snapshots());

                self.backup_repository
                    .create_backup(call.user(), backup)
                    .await
                    .map_err(|e| BackupRepositoryError(e.into()))?;

                call.answer(Response::Successful)
                    .await
                    .map_err(|e| ResponseError(e.into()))?;
            }

            _ => unimplemented!(),
        }

        Ok(())
    }
}

pub enum ServerServiceError {
    BackupRepositoryError(Box<dyn Error>),
    ResponseError(Box<dyn Error>),
    BackupIdNotFound(BackupId),
}
