use clap::{Parser, Subcommand};
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
    Backup(BackupCommands),
}

// In case we need more sophisticated server options
/*#[derive(Subcommand)]
enum ServerCommands {
}*/

#[derive(Subcommand)]
pub enum BackupCommands {
    /// Set rules for automated backup
    Auto {
        /// Set path which will be backed up
        #[arg(short, long)]
        backup_root: PathBuf,
        /// Set how long the backup should be saved (e.g. 30d)
        #[arg(short, long)]
        retention_period: String,
    },
    /// Create a backup and save it to the current server
    Create {
        /// Set path which will be backed up
        #[arg(short, long)]
        backup_root: Option<PathBuf>,
        /// Set how long the backup should be saved (e.g. 30d)
        #[arg(short, long)]
        retention_period: Option<String>,
    },
    /// Restore your files from a backup
    Restore {
        /// Restore the most recent backup in the specified path
        #[arg(short, long)]
        backup_root: PathBuf,
    },
}
