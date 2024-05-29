#![cfg(test)]

use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::ConnectionClientInterface;
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;

pub struct MockConnection();

impl ConnectionClientInterface for MockConnection {
    type Error = ();

    async fn send_request(&mut self, _command: &Call) -> Result<Response, Self::Error> {
        Ok(Response::BackupCreated)
    }

    async fn send_request_with_blob(&mut self, _command: &Call, _blob: impl BlobFetch) -> Result<Response, Self::Error> {
        Ok(Response::BackupCreated)
    }
}
