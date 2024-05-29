use crate::model::connection_interface::{UnhandledIncomingCall};
use crate::server_service::ServerService;

#[derive(Debug, Default)]
pub struct MockServerService {}

impl ServerService for MockServerService {
    type Error = ();

    async fn handle_incoming_request(&mut self, _call: impl UnhandledIncomingCall) -> Result<(), Self::Error> {
        Ok(())
    }
}