use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt, BufStream};
use tokio::net::TcpStream;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::ConnectionClientInterface;
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;

pub struct TcpConnection {
    addr: SocketAddr,
}

impl TcpConnection {
    pub async fn new(addr: SocketAddr) -> Self {
        Self {
            addr
        }
    }
}

impl ConnectionClientInterface for TcpConnection {
    type Error = tokio::io::Error;

    async fn send_request(&mut self, command: &Call) -> Result<Response, Self::Error> {
        let mut encoded = Vec::new();
        ciborium::into_writer(&command, &mut encoded).expect("Vec can always grow");
        
        let mut stream = BufStream::new(TcpStream::connect(self.addr).await?);
        stream.write_u32(encoded.len() as u32).await?;
        stream.write_all(encoded.as_slice()).await?;
        
        todo!()
    }

    async fn send_request_with_blob(&mut self, command: &Call, blob: impl BlobFetch) -> Result<Response, Self::Error> {
        todo!()
    }
}