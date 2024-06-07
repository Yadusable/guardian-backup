use guardian_backup_domain::model::backup::backup::BackupId;
use guardian_backup_domain::model::duration::Duration;
use std::path::PathBuf;

pub struct ClientCommand {
    pub subcommand: ClientSubcommand,
}

pub enum ClientSubcommand {
    /// Add or change to a new Server
    Server {
        /// Set URL of the backup server
        url: String,
        /// Set username on the backup server
        user_name: String,
        /// Set user password on the backup server
        password: String,
    },

    /// Create an (automated) backup, restore from a backup
    Backup(ClientBackupCommand),
}

pub enum ClientBackupCommand {
    /// Set rules for automated backup
    Auto {
        /// Set path which will be backed up
        backup_root: PathBuf,
        /// Set how long the backup should be saved (e.g. 30d)
        retention_period: String,
    },
    /// Create a backup and save it to the current server
    Create {
        /// Set path which will be backed up
        backup_root: PathBuf,
        /// Set how long the backup should be saved (e.g. 30d)
        retention_period: Duration,
        /// Set the interval between two Backups
        interval: Duration,
        /// Set a unique name for the backup to be displayed with
        name: String,
    },
    /// Restore your files from a backup
    Restore {
        /// Restore into the specified path
        backup_root: PathBuf,
        /// Select the most recent [guardian_backup_domain::model::backup::snapshot::Snapshot] of the [BackupId]
        id: BackupId,
    },
    /// List all Backups on the server
    List {},
}
