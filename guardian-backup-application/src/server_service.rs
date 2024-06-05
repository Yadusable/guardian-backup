use crate::model::call::Call;
use crate::model::connection_interface::{IncomingCall, UnhandledIncomingCall};
use crate::model::response::Response;
use crate::server_service::ServerServiceError::{
    BackupIdNotFound, BackupRepositoryError, BlobFetchError, BlobRepositoryError, NoPermission,
    ResponseError,
};
use guardian_backup_domain::model::backup::backup::BackupId;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;
use guardian_backup_domain::repositories::blob_repository::BlobRepository;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait ServerService {
    type Error;

    async fn handle_incoming_request(
        &mut self,
        call: impl UnhandledIncomingCall,
    ) -> Result<(), Self::Error>;
}

pub struct MainServerService<B: BackupRepository, L: BlobRepository> {
    backup_repository: B,
    blob_repository: L,
}

impl<B: BackupRepository, L: BlobRepository> ServerService for MainServerService<B, L> {
    type Error = ServerServiceError;

    async fn handle_incoming_request(
        &mut self,
        call: impl UnhandledIncomingCall,
    ) -> Result<(), Self::Error> {
        let (call_variant, mut call) = call.into_inner();

        if let Err(err) = self.internal_handle(&mut call, call_variant).await {
            call.answer(Response::Error(format!("{err}").into()))
                .await
                .map_err(|e| ResponseError(e.into()))?;
        }

        Ok(())
    }
}

impl<B: BackupRepository, L: BlobRepository> MainServerService<B, L> {
    pub fn new(backup_repository: B, blob_repository: L) -> Self {
        Self {
            backup_repository,
            blob_repository,
        }
    }

    async fn internal_handle(
        &mut self,
        call: &mut impl IncomingCall,
        call_variant: Call,
    ) -> Result<(), ServerServiceError> {
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
                call.answer(Response::BackupList(backups.collect()))
                    .await
                    .map_err(|e| ResponseError(e.into()))?;
            }

            Call::PatchBackup(mut backup) => {
                let origin = self
                    .backup_repository
                    .get_backup_by_id(backup.id(), call.user())
                    .await
                    .map_err(|e| BackupRepositoryError(e.into()))?
                    .ok_or_else(|| BackupIdNotFound(backup.id().clone()))?;

                backup.merge_snapshots(origin.into_snapshots());

                self.backup_repository
                    .update_backup(backup, call.user())
                    .await
                    .map_err(|e| BackupRepositoryError(e.into()))?;

                call.answer(Response::Successful)
                    .await
                    .map_err(|e| ResponseError(e.into()))?;
            }

            Call::CreateBlob(id) => {
                if id.user() != call.user() {
                    return Err(NoPermission);
                }

                let blob = call
                    .receive_blob()
                    .await
                    .map_err(|e| BlobFetchError(e.into()))?;
                self.blob_repository
                    .insert_blob(id, blob)
                    .await
                    .map_err(|e| BlobRepositoryError(e.into()))?;

                call.answer(Response::Successful)
                    .await
                    .map_err(|e| ResponseError(e.into()))?;
            }

            Call::GetBlob(id) => {
                if id.user() != call.user() {
                    return Err(NoPermission);
                }

                let blob = self
                    .blob_repository
                    .fetch_blob(&id)
                    .await
                    .map_err(|e| BlobRepositoryError(e.into()))?;
                call.answer_with_blob(Response::Successful, blob)
                    .await
                    .map_err(|e| ResponseError(e.into()))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ServerServiceError {
    BackupRepositoryError(Box<dyn Error>),
    BlobRepositoryError(Box<dyn Error>),
    ResponseError(Box<dyn Error>),
    BackupIdNotFound(BackupId),
    BlobFetchError(Box<dyn Error>),
    NoPermission,
}

impl Display for ServerServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupRepositoryError(inner) => write!(f, "BackupRepositoryError({inner})"),
            BlobRepositoryError(inner) => write!(f, "BlobRepositoryError({inner})"),
            ResponseError(inner) => write!(f, "ResponseError({inner})"),
            BackupIdNotFound(inner) => write!(f, "BackupIdNotFound({inner})"),
            BlobFetchError(inner) => write!(f, "BlobFetchError({inner})"),
            NoPermission => write!(f, ""),
        }
    }
}
