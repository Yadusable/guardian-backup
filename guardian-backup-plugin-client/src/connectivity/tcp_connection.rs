use std::error::Error;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufStream};
use tokio::net::TcpStream;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{ConnectionClientInterface, IncomingResponse};
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use crate::connectivity::tcp_connection::TcpConnectivityError::{Ciborium, TokioIO};
use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;

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

impl TcpConnection {
    async fn send_request_internal(&mut self, stream: &mut (impl AsyncWrite + Unpin), command: &Call) -> Result<(), <Self as ConnectionClientInterface>::Error> {
        let mut encoded = Vec::new();
        ciborium::into_writer(&command, &mut encoded).expect("Vec can always grow");
        
        stream.write_u32(encoded.len() as u32).await?;
        stream.write_all(encoded.as_slice()).await?;
        Ok(())
    }
    
    async fn receive_response(&mut self, stream: &mut (impl AsyncRead + Unpin)) -> Result<Response, <Self as ConnectionClientInterface>::Error> {
        let response_len = stream.read_u32().await?;
        let response_buf = vec![0; response_len as usize];
        let response = ciborium::de::from_reader(response_buf.as_slice())?;
        Ok(response)
    }
    
    async fn send_blob(&mut self, stream: &mut (impl AsyncWrite + Unpin), mut blob: impl BlobFetch) -> Result<(), <Self as ConnectionClientInterface>::Error>{
        stream.write_u64(blob.total_len()).await?;
        let mut buf = [0; 4096];
        while blob.remaining_len() > 0 {
            let read = blob.read(&mut buf).await.map_err(|e| TcpConnectivityError::BlobFetch(Box::from(e)))?;
            stream.write_all(buf.split_at(read).0).await?
        }
        
        Ok(())
    }
}

impl ConnectionClientInterface for TcpConnection {
    type Error = TcpConnectivityError;

    async fn send_request(&mut self, command: &Call) -> Result<impl IncomingResponse, Self::Error> {
        let mut stream = BufStream::new(TcpStream::connect(self.addr).await?);
        
        self.send_request_internal(&mut stream, command).await?;
        
        let response = self.receive_response(&mut stream).await?;
        
        Ok(IncomingTcpResponse {
            response,
            stream,
        })
        
    }

    async fn send_request_with_blob(&mut self, command: &Call, blob: impl BlobFetch) -> Result<impl IncomingResponse, Self::Error> {
        let mut stream = BufStream::new(TcpStream::connect(self.addr).await?);

        self.send_request_internal(&mut stream, command).await?;
        self.send_blob(&mut stream, blob).await.map_err(|e| TcpConnectivityError::BlobFetch(Box::from(e)))?;

        let response = self.receive_response(&mut stream).await?;

        Ok(
            IncomingTcpResponse{
                response,
                stream
            }
        )
    }
}

pub struct IncomingTcpResponse {
    response: Response,
    stream: BufStream<TcpStream>,
}

impl IncomingResponse for IncomingTcpResponse {
    type Error = tokio::io::Error;

    fn inner(&self) -> &Response {
        &self.response
    }

    async fn receive_blob(mut self) -> Result<impl BlobFetch, Self::Error> {
        let receive_len = self.stream.read_u64().await?;
        let fetch = TokioBlobFetch::new(self.stream, receive_len);
        Ok(fetch)
    }
}

#[derive(Debug)]
pub enum TcpConnectivityError {
    TokioIO(tokio::io::Error),
    Ciborium(ciborium::de::Error<std::io::Error>),
    BlobFetch(Box<dyn std::error::Error>)
}

impl From<tokio::io::Error> for TcpConnectivityError {
    fn from(value: tokio::io::Error) -> Self {
        TokioIO(value)
    }
}

impl From<ciborium::de::Error<std::io::Error>> for TcpConnectivityError {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        Ciborium(value)
    }
}

impl Display for TcpConnectivityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokioIO(inner) => write!(f, "{inner}"),
            Ciborium(inner) => write!(f, "{inner}"),
            TcpConnectivityError::BlobFetch(inner) => write!(f, "{inner}"),
        }
    }
}

impl Error for TcpConnectivityError {}

