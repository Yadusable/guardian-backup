use clap::{Parser, Subcommand};
use guardian_backup_application::model::client_model::{
    ClientBackupCommand, ClientCommand, ClientSubcommand,
};

use guardian_backup_domain::model::backup::backup::BackupId;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Subcommand)]

pub enum EntityType {
    // In case we need more sophisticated server options
    //    /// Add a server
    //    #[clap(subcommand)]
    //    Server(ServerCommands),
    /// Add or change to a new Server
    Server {
        /// Set URL of the backup server
        #[arg(long)]
        url: String,
        /// Set username on the backup server
        #[arg(short, long)]
        user_name: String,
        /// Set user password on the backup server
        #[arg(short, long)]
        password: String,
    },

    /// Create an (automated) backup, restore from a backup
    #[clap(subcommand)]
    Backup(BackupCommand),
}

// In case we need more sophisticated server options
/*#[derive(Subcommand)]
enum ServerCommands {
}*/

#[derive(Subcommand)]
pub enum BackupCommand {
    /// Set rules for automated backup
    Auto {
        /// Create a scheduled Backup
        #[arg(short, long)]
        backup_root: PathBuf,
        /// Set how long a snapshot should be saved (e.g. 30d)
        #[arg(short, long)]
        retention_period: String,
    },
    /// Create a backup and save a snapshot to the current server
    Create {
        /// Set path which will be backed up
        #[arg(short, long)]
        backup_root: PathBuf,
        /// Set how long the backup should be saved (e.g. "3d12h"); default is ~30d
        #[arg(short, long)]
        retention_period: Option<String>,
        /// Set a unique name for the backup to be displayed with
        #[arg(short, long)]
        name: String,
    },
    /// Restore your files from a snapshot
    Restore {
        /// Restore into the specified path
        #[arg(short, long)]
        file_root: PathBuf,
        /// Select the most recent [guardian_backup_domain::model::backup::snapshot::Snapshot] of the [BackupId]
        #[arg(short, long)]
        backup_id: BackupId,
    },
    /// List all Backups on the server
    List {},
}

impl From<Cli> for ClientCommand {
    fn from(value: Cli) -> Self {
        match value {
            Cli { entity_type } => ClientCommand {
                subcommand: entity_type.into(),
            },
        }
    }
}

impl From<EntityType> for ClientSubcommand {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::Server {
                url,
                user_name,
                password,
            } => ClientSubcommand::Server {
                url,
                user_name,
                password,
            },
            EntityType::Backup(inner) => ClientSubcommand::Backup(inner.into()),
        }
    }
}

impl From<BackupCommand> for ClientBackupCommand {
    fn from(value: BackupCommand) -> Self {
        match value {
            BackupCommand::Auto {
                backup_root,
                retention_period,
            } => ClientBackupCommand::Auto {
                backup_root,
                retention_period,
            },
            BackupCommand::Create {
                backup_root,
                retention_period,
                name,
            } => ClientBackupCommand::Create {
                backup_root,
                retention_period,
                name,
            },
            BackupCommand::Restore {
                file_root,
                backup_id,
            } => ClientBackupCommand::Restore {
                backup_root: file_root,
                id: backup_id,
            },
            BackupCommand::List { .. } => {
                todo!()
            }
        }
    }
}
