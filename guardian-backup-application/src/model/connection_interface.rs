use crate::model::call::Call;
use crate::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::fmt::Debug;
use std::future::Future;

pub trait ConnectionClientInterface {
    type Error: std::error::Error + 'static;
    async fn send_request(
        &mut self,
        command: Call,
    ) -> Result<impl IncomingResponse + 'static, Self::Error>;
    async fn send_request_with_blob(
        &mut self,
        command: &Call,
        blob: impl BlobFetch,
    ) -> Result<impl IncomingResponse, Self::Error>;
}

pub trait IncomingResponse: Debug {
    type Error: std::error::Error;
    fn inner(&self) -> &Response;
    fn into_inner(self) -> Response;
    async fn receive_blob(self) -> Result<impl BlobFetch + 'static, Self::Error>;
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
    fn answer(
        &mut self,
        response: Response,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn answer_with_blob(
        &mut self,
        response: Response,
        blob_data: impl BlobFetch,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn user(&self) -> &UserIdentifier;
    fn receive_blob(&mut self) -> impl Future<Output = Result<impl BlobFetch, Self::Error>> + Send;
}
