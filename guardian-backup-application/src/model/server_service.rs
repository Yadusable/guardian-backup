use crate::model::connection_interface::IncomingCall;

pub trait ServerService {
    type Error;
    
    async fn handle_incoming_request<T: IncomingCall>(&self, call: T) -> Result<(), Self::Error>;
}