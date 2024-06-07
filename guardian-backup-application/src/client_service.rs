use crate::client_service::MainClientServiceError::{BlobRepositoryError, FileServiceError};
use crate::encoding_service::EncodingService;
use crate::file_service::File;
use crate::file_service::FileService;
use crate::in_memory_repositories::backup_repository::InMemoryBackupRepository;
use crate::in_memory_repositories::blob_repository::InMemoryBlobFetch;
use crate::in_memory_repositories::blob_repository::InMemoryBlobRepository;
use crate::model::client_model::{ClientBackupCommand, ClientCommand, ClientSubcommand};
use guardian_backup_domain::hash_service::HashService;
use guardian_backup_domain::hash_service::Hasher;
use guardian_backup_domain::hash_service::PendingHashB;
use guardian_backup_domain::model::backup::backup::{Backup, BackupId};
use guardian_backup_domain::model::backup::schedule::Schedule;
use guardian_backup_domain::model::backup::schedule_rule::ScheduleRule;
use guardian_backup_domain::model::backup::snapshot::Snapshot;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::model::device_identifier::DeviceIdentifier;
use guardian_backup_domain::model::duration::Duration;
use guardian_backup_domain::model::files::file_tree::{FileTreeDiffType, FileTreeNode};
use guardian_backup_domain::model::timestamp::Timestamp;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use guardian_backup_domain::repositories::backup_repository::BackupRepository;
use guardian_backup_domain::repositories::blob_repository::BlobRepository;
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::vec;

#[cfg(any(test, feature = "mocks"))]
use crate::model::mocks::mock_encoder_service::MockEncoderService;
#[cfg(any(test, feature = "mocks"))]
use crate::model::mocks::mock_file_service::MockFileService;
#[cfg(any(test, feature = "mocks"))]
use crate::model::mocks::mock_hash_service::MOCK_HASHER;

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
    user: UserIdentifier,
    backup_repository: B,
    blob_repository: L,
    encoding_service: PhantomData<E>,
    file_service: PhantomData<F>,
    hash_service: HashService,
}

impl<B: BackupRepository, L: BlobRepository, E: EncodingService, F: FileService>
    MainClientService<B, L, E, F>
{
    pub fn new(
        user: UserIdentifier,
        backup_repository: B,
        blob_repository: L,
        hash_service: HashService,
    ) -> Self {
        Self {
            user,
            backup_repository,
            blob_repository,
            encoding_service: PhantomData,
            file_service: PhantomData,
            hash_service,
        }
    }
}

#[cfg(any(test, feature = "mocks"))]
impl
    MainClientService<
        InMemoryBackupRepository,
        InMemoryBlobRepository,
        MockEncoderService,
        MockFileService,
    >
{
    pub fn new_mock() -> Self {
        MainClientService {
            user: UserIdentifier::new("Mock".into()),
            backup_repository: InMemoryBackupRepository::new(),
            blob_repository: InMemoryBlobRepository::new(),
            encoding_service: PhantomData,
            file_service: PhantomData,
            hash_service: HashService::new(vec![&MOCK_HASHER as &dyn Hasher]),
        }
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
                ClientBackupCommand::Create {
                    backup_root,
                    retention_period,
                    interval,
                    name,
                } => {
                    self.create_backup(backup_root, retention_period, interval, Box::from(name))
                        .await?;
                    Ok(())
                }
                ClientBackupCommand::Restore { backup_root, id } => {
                    let backup = self
                        .backup_repository
                        .get_backup_by_id(&id, &UserIdentifier::new("local".into()))
                        .await
                        .map_err(|e| MainClientServiceError::BackupRepositoryError(e.into()))?
                        .ok_or(MainClientServiceError::BackupNotFound)?; //TODO correct user handling

                    let mut old_file_tree_blob = self
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
                    let old_file_tree_data = old_file_tree_blob
                        .read_to_eof()
                        .await
                        .map_err(|e| MainClientServiceError::BlobRepositoryError(e.into()))?;
                    let old_file_tree: FileTreeNode = E::decode(old_file_tree_data.as_ref())
                        .map_err(|e| MainClientServiceError::DecodeError(e.into()))?;
                    drop(old_file_tree_blob);
                    drop(old_file_tree_data);

                    let new_file_tree = F::generate_file_tree(
                        backup_root.as_path(),
                        self.hash_service.preferred_hasher(),
                        &self.user,
                    )
                    .await
                    .map_err(|e| MainClientServiceError::FileServiceError(e.into()))?;

                    self.resolve_diffs(new_file_tree, old_file_tree, backup_root.as_path())
                        .await?;
                    Ok(())
                }
                ClientBackupCommand::List {} => {
                    todo!()
                }
            },
        }
    }
}

impl<B: BackupRepository, L: BlobRepository, E: EncodingService, F: FileService>
    MainClientService<B, L, E, F>
{
    async fn create_backup(
        &mut self,
        backup_root: PathBuf,
        retention_period: Duration,
        interval: Duration,
        name: Box<str>,
    ) -> Result<(), MainClientServiceError> {
        let mut schedule = Schedule::new(Vec::new());

        if let Duration::Limited { .. } = interval {
            schedule.add_rule(ScheduleRule::new(
                retention_period.clone(),
                interval,
                Timestamp::now(),
            ));
        }

        let filetree = F::generate_file_tree(
            backup_root.as_path(),
            self.hash_service.preferred_hasher(),
            &self.user,
        )
        .await
        .map_err(|e| MainClientServiceError::FileServiceError(e.into()))?;

        let file_tree_box = E::encode(&filetree);
        let hasher = self.hash_service.preferred_hasher();
        let mut hash = hasher.create_hash();
        hash.update(&file_tree_box);
        let hash = hash.finalize();
        let file_tree_blob = InMemoryBlobFetch::new(file_tree_box.into());

        let file_tree_blob_identifier = BlobIdentifier::new(hash, self.user.clone());
        self.blob_repository
            .insert_blob(file_tree_blob_identifier.clone(), file_tree_blob)
            .await
            .map_err(|e| MainClientServiceError::BlobRepositoryError(e.into()))?;

        let mut blobs = vec![file_tree_blob_identifier.clone()];
        blobs.append(
            &mut self
                .upload_to_repository_from_file_tree(&filetree, backup_root.clone())
                .await?,
        );

        let snapshots = vec![Snapshot::new(
            Timestamp::now(),
            Timestamp::now() + &retention_period,
            file_tree_blob_identifier,
            blobs,
        )];

        let backup = Backup::new(
            BackupId::from_str(name.as_ref()).unwrap(),
            DeviceIdentifier::default(),
            schedule,
            Box::from(backup_root),
            snapshots,
        );

        self.backup_repository
            .create_backup(&self.user, backup)
            .await
            .map_err(|e| MainClientServiceError::BackupRepositoryError(e.into()))?;
        Ok(())
    }

    async fn upload_to_repository_from_file_tree(
        &mut self,
        file_tree_node: &FileTreeNode,
        backup_root_path: PathBuf,
    ) -> Result<Vec<BlobIdentifier>, MainClientServiceError> {
        let files = file_tree_node.iter(backup_root_path.parent().unwrap().into());
        let mut associated_blobs = vec![];
        for (path, file_node) in files {
            if let FileTreeNode::File {
                name,
                blob: blob_identifier,
                ..
            } = file_node
            {
                let file = F::get_file(path.join(name).as_path())
                    .await
                    .map_err(|e| MainClientServiceError::FileServiceError(e.into()))?;
                let blob = file
                    .get_as_blob()
                    .await
                    .map_err(|e| MainClientServiceError::FileServiceError(e.into()))?;
                self.blob_repository
                    .insert_blob(blob_identifier.clone(), blob)
                    .await
                    .map_err(|e| MainClientServiceError::BlobRepositoryError(e.into()))?;
                associated_blobs.push(blob_identifier.clone())
            }
        }
        Ok(associated_blobs)
    }
}

impl<B: BackupRepository, L: BlobRepository, E: EncodingService, F: FileService>
    MainClientService<B, L, E, F>
{
    pub async fn resolve_diffs(
        &mut self,
        current_state: FileTreeNode,
        expected_state: FileTreeNode,
        root: &Path,
    ) -> Result<(), MainClientServiceError> {
        let diffs = expected_state.diff_to(&current_state, root.into());

        for diff in diffs {
            match diff.diff_type {
                FileTreeDiffType::Created => {
                    self.recursive_create_in_fs(diff.location.as_ref(), &diff.node)
                        .await?;
                }
                FileTreeDiffType::Updated => {
                    if let FileTreeNode::File {
                        name,
                        blob,
                        metadata,
                    } = diff.node
                    {
                        F::write_file(
                            diff.location.join(name).as_path(),
                            &metadata,
                            self.blob_repository
                                .fetch_blob(&blob)
                                .await
                                .map_err(|e| BlobRepositoryError(e.into()))?,
                        )
                        .await
                        .map_err(|e| FileServiceError(e.into()))?
                    }
                }
                FileTreeDiffType::Deleted => match diff.node {
                    FileTreeNode::File { name, .. } => {
                        F::delete_file(diff.location.join(name).as_path())
                            .await
                            .map_err(|e| FileServiceError(e.into()))?
                    }
                    FileTreeNode::Directory { name, .. } => {
                        F::delete_dir_all(diff.location.join(name).as_path())
                            .await
                            .map_err(|e| FileServiceError(e.into()))?
                    }
                    FileTreeNode::SymbolicLink { .. } => {
                        unimplemented!()
                    }
                },
                FileTreeDiffType::ChangedType => {
                    F::delete_dir_all(diff.location.join(diff.node.name()).as_path())
                        .await
                        .map_err(|e| FileServiceError(e.into()))?;

                    match diff.node {
                        FileTreeNode::File {
                            name,
                            blob,
                            metadata,
                        } => F::write_file(
                            diff.location.join(name).as_path(),
                            &metadata,
                            self.blob_repository
                                .fetch_blob(&blob)
                                .await
                                .map_err(|e| BlobRepositoryError(e.into()))?,
                        )
                        .await
                        .map_err(|e| FileServiceError(e.into()))?,
                        FileTreeNode::Directory {
                            name,
                            metadata,
                            children,
                        } => F::create_dir(diff.location.join(name).as_path())
                            .await
                            .map_err(|e| FileServiceError(e.into()))?,
                        FileTreeNode::SymbolicLink { .. } => {
                            unimplemented!()
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn recursive_create_in_fs(
        &mut self,
        path: &Path,
        dir: &FileTreeNode,
    ) -> Result<(), MainClientServiceError> {
        match dir {
            FileTreeNode::File {
                name,
                metadata,
                blob,
            } => F::write_file(
                path.join(name).as_path(),
                metadata,
                self.blob_repository
                    .fetch_blob(blob)
                    .await
                    .map_err(|e| BlobRepositoryError(e.into()))?,
            )
            .await
            .map_err(|e| FileServiceError(e.into()))?,
            FileTreeNode::Directory { name, children, .. } => {
                F::create_dir(path.join(name).as_path())
                    .await
                    .map_err(|e| FileServiceError(e.into()))?;

                for child in children {
                    Box::pin(self.recursive_create_in_fs(path.join(name).as_path(), child)).await?;
                }
            }
            FileTreeNode::SymbolicLink { .. } => {
                unimplemented!()
            }
        }

        Ok(())
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

#[derive(Debug)]
pub enum CreateErrors {
    InvalidRoot,
}

impl Display for CreateErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateErrors::InvalidRoot => write!(f, "The provided root path was invalid"),
        }
    }
}

impl Error for CreateErrors {}

#[cfg(test)]
mod tests {
    use crate::client_service::{ClientService, MainClientService};
    use crate::model::client_model::ClientBackupCommand::Create;
    use crate::model::client_model::{ClientCommand, ClientSubcommand};
    use guardian_backup_domain::model::backup::backup::{Backup, BackupId};
    use guardian_backup_domain::model::backup::schedule::Schedule;
    use guardian_backup_domain::model::backup::schedule_rule::ScheduleRule;
    use guardian_backup_domain::model::backup::snapshot::Snapshot;
    use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
    use guardian_backup_domain::model::device_identifier::DeviceIdentifier;
    use guardian_backup_domain::model::duration::{Duration, MONTH};
    use guardian_backup_domain::model::files::file_hash::FileHash;
    use guardian_backup_domain::model::timestamp::Timestamp;
    use guardian_backup_domain::model::user_identifier::UserIdentifier;
    use guardian_backup_domain::repositories::backup_repository::BackupRepository;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_if_create_backup_creates_backup() {
        let mut client_service = MainClientService::new_mock();
        client_service
            .create_backup(PathBuf::new(), MONTH, Duration::Infinite, "Testname".into())
            .await
            .unwrap();
        let backups_repo = client_service
            .backup_repository
            .get_backup_by_id(&BackupId("Testname".into()), &client_service.user)
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn test_if_create_backup_contains_right_backup() {
        let mut client_service = MainClientService::new_mock();
        client_service
            .create_backup(
                PathBuf::new(),
                Duration::Infinite,
                Duration::Infinite,
                "Testname".into(),
            )
            .await
            .unwrap();
        let backups_repo = client_service
            .backup_repository
            .get_backup_by_id(&BackupId("Testname".into()), &client_service.user)
            .await
            .unwrap()
            .unwrap();

        let expected_backup = Backup::new(
            BackupId("Testname".into()),
            DeviceIdentifier::default(),
            Schedule::new(vec![]),
            PathBuf::new().into(),
            vec![Snapshot::new(
                Timestamp::now(),
                None,
                BlobIdentifier::new(FileHash::Mock, client_service.user.clone()),
                vec![BlobIdentifier::new(FileHash::Mock, client_service.user)],
            )],
        );
        assert_eq!(backups_repo, expected_backup);
    }

    #[tokio::test]
    async fn test_if_create_backup_contains_right_backup_with_retention() {
        let mut client_service = MainClientService::new_mock();
        client_service
            .create_backup(PathBuf::new(), MONTH, Duration::Infinite, "Testname".into())
            .await
            .unwrap();
        let backups_repo = client_service
            .backup_repository
            .get_backup_by_id(&BackupId("Testname".into()), &client_service.user)
            .await
            .unwrap()
            .unwrap();

        let expected_backup = Backup::new(
            BackupId("Testname".into()),
            DeviceIdentifier::default(),
            Schedule::new(vec![]),
            PathBuf::new().into(),
            vec![Snapshot::new(
                Timestamp::now(),
                Timestamp::now() + &MONTH,
                BlobIdentifier::new(FileHash::Mock, client_service.user.clone()),
                vec![BlobIdentifier::new(FileHash::Mock, client_service.user)],
            )],
        );
        assert_eq!(backups_repo, expected_backup);
    }
}
