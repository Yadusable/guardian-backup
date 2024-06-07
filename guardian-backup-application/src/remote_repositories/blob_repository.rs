use crate::model::call::Call;
use crate::model::connection_interface::ConnectionClientInterface;
use crate::model::connection_interface::IncomingResponse;
use crate::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::blobs::blob_identifier::BlobIdentifier;
use guardian_backup_domain::repositories::blob_repository::BlobRepository;
use std::fmt::{Display, Formatter};

pub struct RemoteBlobRepository<C: ConnectionClientInterface> {
    connectivity_service: C,
}

impl<C: ConnectionClientInterface> RemoteBlobRepository<C> {
    pub fn new(connectivity_service: C) -> Self {
        Self {
            connectivity_service,
        }
    }
}

impl<C: ConnectionClientInterface> BlobRepository for RemoteBlobRepository<C> {
    type Error = RemoteBlobRepositoryError;

    async fn insert_blob(
        &mut self,
        id: BlobIdentifier,
        blob: impl BlobFetch,
    ) -> Result<(), Self::Error> {
        let call = Call::CreateBlob(id);
        let res = self
            .connectivity_service
            .send_request_with_blob(&call, blob)
            .await
            .map_err(|e| RemoteBlobRepositoryError::Connectivity(e.into()))?;

        if &Response::Successful != res.inner() {
            println!("{:?}", res.inner());
            todo!()
        }

        Ok(())
    }

    async fn delete_blob(&mut self, id: &BlobIdentifier) -> Result<(), Self::Error> {
        panic!("Never call delete on a remote blob repository")
    }

    async fn fetch_blob(&mut self, id: &BlobIdentifier) -> Result<impl BlobFetch, Self::Error> {
        let call = Call::GetBlob(id.clone());
        let res = self
            .connectivity_service
            .send_request(call)
            .await
            .map_err(|e| RemoteBlobRepositoryError::Connectivity(e.into()))?;

        if &Response::Successful != res.inner() {
            println!("{:?}", res.inner());
            todo!()
        }

        let res_blob = res
            .receive_blob()
            .await
            .map_err(|e| RemoteBlobRepositoryError::IncomingRequest(e.into()))?;
        Ok(res_blob)
    }
}

#[derive(Debug)]
pub enum RemoteBlobRepositoryError {
    Connectivity(Box<dyn std::error::Error>),
    IncomingRequest(Box<dyn std::error::Error>),
}

impl Display for RemoteBlobRepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoteBlobRepositoryError::Connectivity(inner) => write!(f, "Connectivity({inner})"),
            RemoteBlobRepositoryError::IncomingRequest(inner) => {
                write!(f, "IncomingRequest({inner})")
            }
        }
    }
}

impl std::error::Error for RemoteBlobRepositoryError {}
