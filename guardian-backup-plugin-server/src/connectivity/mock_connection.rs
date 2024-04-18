#![cfg(test)]

use std::path::PathBuf;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{ConnectionServerInterface, IncomingCall};
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::model::backup::backup::Backup;
use guardian_backup_domain::model::backup::schedule::Schedule;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::device_identifier::DeviceIdentifier;

#[derive(Default, Debug)]
pub struct MockConnection();

impl ConnectionServerInterface for MockConnection {
    type Error = ();
    type Call = IncomingMockCall;

    async fn receive_request(&mut self) -> Result<Self::Call, Self::Error> {
        Ok(IncomingMockCall{inner:Call::CreateBackup(Backup::new(DeviceIdentifier::default(), Schedule::default(), PathBuf::from("tmp/mocks/").into(), Vec::default()))})
    }
}

pub struct IncomingMockCall {
    inner:Call
}

impl IncomingCall for IncomingMockCall {
    type Error = ();

    async fn answer<T: BlobFetch>(self, _response: Response, _blob_data: Option<T>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn inner(&self) -> &Call {
        &self.inner
    }
}
