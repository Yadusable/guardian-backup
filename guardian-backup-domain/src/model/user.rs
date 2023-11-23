use crate::model::backup::Backup;

pub struct User {
    identifier: Box<str>,
    backups: Vec<Backup>,
}