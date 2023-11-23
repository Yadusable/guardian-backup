use std::path::Path;
use crate::model::device::Device;
use crate::model::schedule::Schedule;
use crate::model::snapshot::Snapshot;

pub struct Backup {
    device: Device,
    schedule: Schedule,
    file_root: Box<Path>,
    snapshots: Vec<Snapshot>,
}