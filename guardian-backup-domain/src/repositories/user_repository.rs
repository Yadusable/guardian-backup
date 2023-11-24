use crate::model::device_identifier::DeviceIdentifier;
use crate::model::error::AsyncResult;
use crate::model::user::User;
use crate::model::user_identifier::UserIdentifier;

pub trait UserRepository {
    fn get_user(&self, identifier: &UserIdentifier) -> AsyncResult<User>;
    fn get_user_devices(&self, user: &UserIdentifier) -> AsyncResult<Box<[DeviceIdentifier]>>;

    fn create_user(&mut self, user: &User) -> AsyncResult<()>;
    fn create_user_device(
        &mut self,
        user: &UserIdentifier,
        device: &DeviceIdentifier,
    ) -> AsyncResult<()>;

    fn delete_user(&mut self, identifier: &UserIdentifier) -> AsyncResult<()>;
    fn delete_device(&mut self, user: &UserIdentifier, device: &DeviceIdentifier);
}
