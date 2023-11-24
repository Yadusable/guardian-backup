use crate::model::backup::schedule::Schedule;
use crate::model::backup::snapshot::Snapshot;
use crate::model::device_identifier::DeviceIdentifier;
use std::path::Path;

#[derive(Debug)]
pub struct Backup {
    device: DeviceIdentifier,
    schedule: Schedule,
    file_root: Box<Path>,
    snapshots: Vec<Snapshot>,
}

impl Backup {
    pub fn new(
        device: DeviceIdentifier,
        schedule: Schedule,
        file_root: Box<Path>,
        snapshots: Vec<Snapshot>,
    ) -> Self {
        Self {
            device,
            schedule,
            file_root,
            snapshots,
        }
    }

    pub fn device(&self) -> &DeviceIdentifier {
        &self.device
    }
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
    pub fn file_root(&self) -> &Box<Path> {
        &self.file_root
    }
    pub fn snapshots(&self) -> &Vec<Snapshot> {
        &self.snapshots
    }
}
