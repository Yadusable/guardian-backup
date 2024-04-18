use crate::model::connection_interface::IncomingCall;
use crate::model::server_service::ServerService;

#[derive(Debug, Default)]
pub struct MockServerService {}

impl ServerService for MockServerService {
    type Error = ();

    async fn handle_incoming_request<T: IncomingCall>(&self, _call: T) -> Result<(), Self::Error> {
        Ok(())
    }
}