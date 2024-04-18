#![cfg(test)]

use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::ConnectionClientInterface;
use guardian_backup_application::model::response::Response;

pub struct MockConnection();

impl ConnectionClientInterface for MockConnection {
    type Error = ();

    async fn send_request(&mut self, _command: Call) -> Result<Response, Self::Error> {
        Ok(Response::BackupCreated)
    }
}
