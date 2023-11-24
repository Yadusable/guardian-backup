use crate::model::backup::backup::Backup;
use crate::model::error::AsyncResult;
use crate::model::user::User;

pub trait BackupRepository {
    fn get_backups(&self, user: &User) -> AsyncResult<Box<[Backup]>>;
}
