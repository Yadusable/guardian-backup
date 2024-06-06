use crate::encoding_service::EncodingService;
use crate::file_service::FileService;
use crate::model::client_model::{ClientBackupCommand, ClientCommand, ClientSubcommand};
use guardian_backup_domain::hash_service::HashService;
use guardian_backup_domain::model::backup::backup::{Backup, BackupId};
use guardian_backup_domain::model::backup::schedule::Schedule;
use guardian_backup_domain::model::backup::schedule_rule::ScheduleRule;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::device_identifier::DeviceIdentifier;
use guardian_backup_domain::model::duration::Duration;
use guardian_backup_domain::model::files::file_tree::{
    FileTreeDiff, FileTreeDiffType, FileTreeNode,
};
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
                ClientBackupCommand::Create {
                    backup_root,
                    retention_period,
                    name,
                } => {
                    create_backup(backup_root.unwrap(), retention_period, Box::from(name)).await;
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

                    self.resolve_diffs(new_file_tree, old_file_tree, backup_root.as_path());
                    Ok(())
                }
                ClientBackupCommand::List {} => {
                    todo!()
                }
            },
        }
    }
}

async fn create_backup(backup_root: PathBuf, retention_period: Option<String>, name: Box<str>) {
    let mut schedule = Schedule::new(Vec::new());

    let mut ret_period = 2600000;
    match retention_period {
        None => {}
        Some(_) => {
            let seconds_unwrapped = &*retention_period.unwrap();
            if let Ok(duration_seconds) = duration_to_seconds(seconds_unwrapped) {
                ret_period = duration_seconds * 1000
            }
        }
    }

    schedule.add_rule(ScheduleRule::new(
        Duration::Limited {
            milliseconds: ret_period as u64,
        },
        Duration::Infinite,
        Timestamp::now(),
    ));
    Backup::new(
        BackupId::from_str(name.as_ref()).unwrap(),
        DeviceIdentifier::default(),
        schedule,
        Box::from(backup_root),
        Vec::new(),
    );
}

fn duration_to_seconds(input: &str) -> Result<u32, DurationErrors> {
    let input_str = input;
    let regex = Regex::new(r"(\d+d|\d+h|\d+m)").unwrap();
    println!("{}", input_str);

    let mut duration_in_sec = 0;

    if !regex.is_match(input_str) {
        return Err(DurationErrors::NoMatches);
    }

    for timepart_capture in regex.captures_iter(input_str) {
        let time_piece = timepart_capture.get(0).unwrap().as_str();
        println!("{:?}", time_piece);
        let (time_amount_str, unit) = time_piece.split_at(time_piece.len() - 1);
        let time_amount = time_amount_str.parse::<u32>().unwrap();
        match unit {
            "d" => {
                duration_in_sec += 24 * 60 * 60 * time_amount;
            }
            "h" => {
                duration_in_sec += 60 * 60 * time_amount;
            }
            "m" => {
                duration_in_sec += 60 * time_amount;
            }
            _ => {
                panic!("should be unreachable, check duration regex")
            }
        }
    }
    Ok(duration_in_sec)
}

#[derive(Debug)]
pub enum DurationErrors {
    NoMatches,
}

impl Display for DurationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationErrors::NoMatches => write!(f, "No matches in the valid format found!"),
        }
    }
}

impl Error for DurationErrors {}

impl<B: BackupRepository, L: BlobRepository, E: EncodingService, F: FileService>
    MainClientService<B, L, E, F>
{
    pub fn resolve_diffs(
        &self,
        current_state: FileTreeNode,
        expected_state: FileTreeNode,
        root: &Path,
    ) {
        let diffs = expected_state.diff_to(&current_state, root.into());

        for diff in diffs {
            todo!();
            match diff.diff_type {
                FileTreeDiffType::Created => {}
                FileTreeDiffType::Updated => {}
                FileTreeDiffType::Deleted => {}
                FileTreeDiffType::ChangedType => {}
            }
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
