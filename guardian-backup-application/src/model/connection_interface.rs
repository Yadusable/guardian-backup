use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use crate::model::call::Call;
use crate::model::response::Response;

pub trait ConnectionClientInterface {
    type Error;
    async fn send_request(&mut self, command: Call) -> Result<Response, Self::Error>;
}
pub trait ConnectionServerInterface {
    type Error;
    type Call: IncomingCall;
    async fn receive_request(&mut self) -> Result<Self::Call, Self::Error>;
}
pub trait IncomingCall {
    type Error;
    async fn answer<T: BlobFetch>(self, response: Response, blob_data: Option<T>) -> Result<(), Self::Error>;
    fn inner(&self) -> &Call;
}
