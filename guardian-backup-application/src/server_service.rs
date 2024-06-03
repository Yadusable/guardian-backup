use crate::model::call::Call;
use crate::model::connection_interface::{IncomingCall, UnhandledIncomingCall};
use crate::model::response::Response::BackupCreated;
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
                    .map_err(|err| ServerServiceError::BackupRepositoryError(err.into()))?;
                call.answer(BackupCreated)
                    .await
                    .map_err(|err| ServerServiceError::ResponseError(err.into()))?;
            }
        }

        Ok(())
    }
}

pub enum ServerServiceError {
    BackupRepositoryError(Box<dyn Error>),
    ResponseError(Box<dyn Error>),
}
