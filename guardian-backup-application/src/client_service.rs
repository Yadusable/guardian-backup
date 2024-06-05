use crate::encoding_service::EncodingService;
use crate::file_service::FileService;
use crate::model::client_model::{ClientBackupCommand, ClientCommand, ClientSubcommand};
use guardian_backup_domain::hash_service::HashService;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::files::file_tree::FileTreeNode;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;
use guardian_backup_domain::repositories::blob_repository::BlobRepository;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub trait ClientService {
    type Error: Error;

    async fn handle_command(&mut self, command: ClientCommand) -> Result<(), Self::Error>;
}

pub struct MainClientService<
    B: BackupRepository,
    L: BlobRepository,
    E: EncodingService,
    F: FileService,
> {
    backup_repository: B,
    blob_repository: L,
    encoding_service: PhantomData<E>,
    file_service: PhantomData<F>,
    hash_service: HashService,
}

impl<B: BackupRepository, L: BlobRepository, E: EncodingService, F: FileService>
    MainClientService<B, L, E, F>
{
    pub fn new() -> Self {
        todo!()
    }
}

impl<B: BackupRepository, L: BlobRepository, E: EncodingService, F: FileService> ClientService
    for MainClientService<B, L, E, F>
{
    type Error = MainClientServiceError;

    async fn handle_command(&mut self, command: ClientCommand) -> Result<(), Self::Error> {
        match command.subcommand {
            ClientSubcommand::Server { .. } => {
                unimplemented!()
            }
            ClientSubcommand::Backup(inner) => match inner {
                ClientBackupCommand::Auto { .. } => {
                    todo!()
                }
                ClientBackupCommand::Create { .. } => {
                    todo!()
                }
                ClientBackupCommand::Restore { backup_root, id } => {
                    let backup = self
                        .backup_repository
                        .get_backup_by_id(&id, &UserIdentifier::new("local".into()))
                        .await
                        .map_err(|e| MainClientServiceError::BackupRepositoryError(e.into()))?
                        .ok_or(MainClientServiceError::BackupNotFound)?; //TODO correct user handling

                    let mut file_tree_blob = self
                        .blob_repository
                        .fetch_blob(
                            backup
                                .snapshots()
                                .last()
                                .ok_or(MainClientServiceError::SnapshotNotFound)?
                                .file_tree_blob(),
                        )
                        .await
                        .map_err(|e| MainClientServiceError::BlobRepositoryError(e.into()))?;
                    let file_tree_blob = file_tree_blob
                        .read_to_eof()
                        .await
                        .map_err(|e| MainClientServiceError::BlobRepositoryError(e.into()))?;
                    let file_tree: FileTreeNode = E::decode(file_tree_blob.as_ref())
                        .map_err(|e| MainClientServiceError::DecodeError(e.into()))?;

                    let diffs = F::compare_to_file_tree(backup_root.as_path(), &file_tree)
                        .await
                        .map_err(|e| MainClientServiceError::FileServiceError(e.into()))?;
                    todo!()
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum MainClientServiceError {
    BackupNotFound,
    SnapshotNotFound,
    FileServiceError(Box<dyn Error>),
    DecodeError(Box<dyn Error>),
    FailReceiveBlob(Box<dyn Error>),
    BackupRepositoryError(Box<dyn Error>),
    BlobRepositoryError(Box<dyn Error>),
}

impl Display for MainClientServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MainClientServiceError::BackupRepositoryError(err) => {
                write!(f, "BackupRepositoryError({err})")
            }
            MainClientServiceError::BlobRepositoryError(err) => {
                write!(f, "BlobRepositoryError({err})")
            }
            MainClientServiceError::BackupNotFound => write!(f, "BackupID not found"),
            MainClientServiceError::SnapshotNotFound => write!(f, "SnapshotNotFound"),
            MainClientServiceError::FailReceiveBlob(err) => {
                write!(f, "Failed to receive BLOB ({err})")
            }
            MainClientServiceError::DecodeError(err) => write!(f, "Failed to decode ({err})"),
            MainClientServiceError::FileServiceError(err) => write!(f, "FileServiceError({err})"),
        }
    }
}

impl Error for MainClientServiceError {}
