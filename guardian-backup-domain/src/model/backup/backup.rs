use crate::model::backup::schedule::Schedule;
use crate::model::backup::snapshot::Snapshot;
use crate::model::device_identifier::DeviceIdentifier;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::path::Path;
use std::str::FromStr;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Backup {
    id: BackupId,
    device: DeviceIdentifier,
    schedule: Schedule,
    file_root: Box<Path>,
    snapshots: Vec<Snapshot>,
}

impl Backup {
    pub fn new(
        id: BackupId,
        device: DeviceIdentifier,
        schedule: Schedule,
        file_root: Box<Path>,
        snapshots: Vec<Snapshot>,
    ) -> Self {
        Self {
            id,
            device,
            schedule,
            file_root,
            snapshots,
        }
    }

    #[cfg(any(test, feature = "mocks"))]
    pub fn mock() -> Self {
        static MOCK_BACKUP_COUNTER: AtomicU16 = AtomicU16::new(1);

        Self::new(
            BackupId(format!("MockBackup_{}", MOCK_BACKUP_COUNTER.fetch_add(1, SeqCst)).into()),
            DeviceIdentifier::default(),
            Schedule::default(),
            Path::new("/mockPath").into(),
            Vec::new(),
        )
    }

    pub fn add_snapshot(&mut self, new_snap: Snapshot) {
        self.snapshots.push(new_snap);
    }

    pub fn device(&self) -> &DeviceIdentifier {
        &self.device
    }
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
    pub fn file_root(&self) -> &Path {
        &self.file_root
    }
    pub fn snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }
    pub fn into_snapshots(self) -> impl IntoIterator<Item = Snapshot> {
        self.snapshots.into_iter()
    }
    pub fn id(&self) -> &BackupId {
        &self.id
    }

    pub fn merge_snapshots(&mut self, snapshots: impl IntoIterator<Item = Snapshot>) {
        for snapshot in snapshots {
            if !self.snapshots.contains(&snapshot) {
                self.snapshots.push(snapshot)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct BackupId(pub Box<str>);

impl FromStr for BackupId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}
