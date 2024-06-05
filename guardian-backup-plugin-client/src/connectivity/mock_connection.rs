#![cfg(test)]

use guardian_backup_application::in_memory_repositories::blob_repository::InMemoryBlobFetch;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{
    ConnectionClientInterface, IncomingResponse,
};
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct MockConnection();

impl ConnectionClientInterface for MockConnection {
    type Error = ();

    async fn send_request(
        &mut self,
        _command: &Call,
    ) -> Result<impl IncomingResponse, Self::Error> {
        Ok(MockIncomingResponse {
            response: Response::Successful,
            blob: None,
        })
    }

    async fn send_request_with_blob(
        &mut self,
        _command: &Call,
        _blob: impl BlobFetch,
    ) -> Result<impl IncomingResponse, Self::Error> {
        Ok(MockIncomingResponse {
            response: Response::Successful,
            blob: None,
        })
    }
}

#[derive(Debug)]
pub struct MockIncomingResponse {
    response: Response,
    blob: Option<InMemoryBlobFetch>,
}

impl IncomingResponse for MockIncomingResponse {
    type Error = IncomingMockError;

    fn inner(&self) -> &Response {
        &self.response
    }

    async fn receive_blob(self) -> Result<impl BlobFetch, Self::Error> {
        self.blob.ok_or(IncomingMockError::NoBLOB)
    }
}

#[derive(Debug)]
pub enum IncomingMockError {
    NoBLOB,
}

impl Display for IncomingMockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomingMockError::NoBLOB => write!(f, "NoBLOB"),
        }
    }
}

impl Error for IncomingMockError {}
