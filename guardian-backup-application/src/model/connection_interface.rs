use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use crate::model::call::Call;
use crate::model::response::Response;

pub trait ConnectionClientInterface {
    type Error;
    async fn send_request(&mut self, command: Call) -> Result<Response, Self::Error>;
}

pub trait ConnectionServerInterface {
    type Error: std::error::Error;
    type Call: IncomingCall;
    async fn receive_request(&mut self) -> Result<impl UnhandledIncomingCall, Self::Error>;
}

pub trait UnhandledIncomingCall: IncomingCall {
    fn into_inner(self) -> (Call, impl IncomingCall);
    fn inner(&self) -> &Call;
}

pub trait IncomingCall {
    type Error: 'static + std::error::Error;
    async fn answer(self, response: Response) -> Result<(), Self::Error>;
    async fn answer_with_blob(self, response: Response, blob_data: impl BlobFetch) -> Result<(), Self::Error>;
    fn user(&self) -> &UserIdentifier;
}
