use std::path::{Path};
use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::backup::schedule::Schedule;
use guardian_backup_domain::model::backup::snapshot::Snapshot;
use crate::model::client_config::ClientConfig;

fn newBackup (path: Box<Path>, config: &ClientConfig) {
    let snaps:Vec<Snapshot> = Vec::new();
    Backup::new(config.device_id.clone(), Schedule::default(), path, snaps);
}

// create Rule

//delete Rule

//show Snapshots

