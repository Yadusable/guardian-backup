use crate::model::device_identifier::DeviceIdentifier;
use crate::model::user::User;
use crate::model::user_identifier::UserIdentifier;
use std::borrow::Cow;

pub trait UserRepository {
    type Error;

    async fn get_user(&self, identifier: &UserIdentifier) -> Result<User, Self::Error>;
    async fn get_user_devices(
        &self,
        user: &UserIdentifier,
    ) -> Result<Cow<[DeviceIdentifier]>, Self::Error>;

    async fn create_user(&mut self, user: &User) -> Result<(), Self::Error>;
    async fn create_user_device(
        &mut self,
        user: &UserIdentifier,
        device: &DeviceIdentifier,
    ) -> Result<(), Self::Error>;

    async fn delete_user(&mut self, identifier: &UserIdentifier) -> Result<(), Self::Error>;
    async fn delete_device(
        &mut self,
        user: &UserIdentifier,
        device: &DeviceIdentifier,
    ) -> Result<(), Self::Error>;
}
