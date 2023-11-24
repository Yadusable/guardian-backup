use crate::model::backup::backup::Backup;
use crate::model::user_identifier::UserIdentifier;

#[derive(Debug)]
pub struct User {
    identifier: UserIdentifier,
    backups: Vec<Backup>,
}

impl User {
    pub fn new(identifier: UserIdentifier, backups: Vec<Backup>) -> Self {
        Self {
            identifier,
            backups,
        }
    }

    pub fn identifier(&self) -> &UserIdentifier {
        &self.identifier
    }
    pub fn backups(&self) -> &[Backup] {
        self.backups.as_slice()
    }
}
