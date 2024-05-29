#![cfg(test)]

use std::convert::Infallible;
use std::path::PathBuf;
use guardian_backup_application::in_memory_repositories::blob_repository::InMemoryBlobFetch;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{ConnectionServerInterface, IncomingCall, UnhandledIncomingCall};
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::helper::{CNone, COptional, CSome};
use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::backup::schedule::Schedule;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::device_identifier::DeviceIdentifier;
use guardian_backup_domain::model::user_identifier::UserIdentifier;

#[derive(Default, Debug)]
pub struct MockConnection();

impl ConnectionServerInterface for MockConnection {
    type Error = Infallible;
    type Call = IncomingMockCall<CSome<Call>>;

    async fn receive_request(&mut self) -> Result<impl UnhandledIncomingCall, Self::Error> {
        Ok(IncomingMockCall::new(Call::CreateBackup(Backup::new(DeviceIdentifier::default(), Schedule::default(), PathBuf::from("tmp/mocks/").into(), Vec::default()))))
    }
}

pub struct IncomingMockCall<CallHandled: COptional<Item=Call>> {
    inner: CallHandled,
    user: UserIdentifier,
}

impl IncomingMockCall<CNone<Call>> {
    pub fn new(call: Call) -> IncomingMockCall<CSome<Call>> {
        IncomingMockCall{inner: CSome(call), user: UserIdentifier::new("MockUser".into())}
    }
}

impl<CallHandled: COptional<Item=Call>> IncomingCall for IncomingMockCall<CallHandled> {
    type Error = Infallible;

    async fn answer(self, _response: Response) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn answer_with_blob(self, _response: Response, _blob_data: impl BlobFetch) -> Result<(), Self::Error> {
        Ok(())
    }

    fn user(&self) -> &UserIdentifier {
        &self.user
    }

    async fn receive_blob(&mut self) -> Result<impl BlobFetch, Self::Error> {
        Ok(InMemoryBlobFetch::new([].into()))
    }
}

impl UnhandledIncomingCall for IncomingMockCall<CSome<Call>> {
    fn into_inner(self) -> (Call, impl IncomingCall) {
        (
            self.inner.0,
            IncomingMockCall{inner: CNone::default(), user: self.user},
        )
    }

    fn inner(&self) -> &Call {
        &self.inner.0
    }
}
