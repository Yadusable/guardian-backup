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
    pub fn new(addr: SocketAddr) -> Self {
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
        let mut response_buf = vec![0; response_len as usize];
        stream.read_exact(response_buf.as_mut_slice()).await?;
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
        stream.write_u64(0).await?; // indicate zero sized blob
        stream.flush().await?;
        
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
        stream.flush().await?;

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
    type Error = TcpConnectivityError;

    fn inner(&self) -> &Response {
        &self.response
    }

    async fn receive_blob(mut self) -> Result<impl BlobFetch, Self::Error> {
        let receive_len = self.stream.read_u64().await?;
        if receive_len == 0 {
            return Err(TcpConnectivityError::NoBlob)
        }
        let fetch = TokioBlobFetch::new(self.stream, receive_len);
        Ok(fetch)
    }
}

#[derive(Debug)]
pub enum TcpConnectivityError {
    TokioIO(tokio::io::Error),
    Ciborium(ciborium::de::Error<std::io::Error>),
    BlobFetch(Box<dyn Error>),
    NoBlob,
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
            TcpConnectivityError::NoBlob => write!(f, "NoBLOB")
        }
    }
}

impl Error for TcpConnectivityError {}

#[cfg(test)]
mod tests {
    use guardian_backup_application::model::call::Call;
    use guardian_backup_application::model::connection_interface::{ConnectionClientInterface, ConnectionServerInterface};
    use guardian_backup_application::server_config::ServerConfig;
    use guardian_backup_domain::model::backup::backup::Backup;
    use guardian_backup_plugin_server::connectivity::tcp_connectivity::TcpServerConnectivity;
    use crate::connectivity::tcp_connection::TcpConnection;
    use guardian_backup_application::model::connection_interface::UnhandledIncomingCall;
    use guardian_backup_application::model::connection_interface::IncomingCall;
    use guardian_backup_application::model::response::Response;
    use guardian_backup_application::model::connection_interface::IncomingResponse;

    async fn run_server(mut server: TcpServerConnectivity, expected_call: Call)  {
            let mut incoming = server.receive_request().await.unwrap();
            assert_eq!(incoming.inner(), &expected_call);
            incoming.receive_blob().await.err().expect("Expected to not receive any blob");

            incoming.answer(Response::BackupCreated).await.unwrap();
    }

    #[tokio::test]
    async fn test_send_request() {
        let server_config = ServerConfig::test_config();
        let server_socket = server_config.bind_to;
        let backup = Backup::mock();
        let call = Call::CreateBackup(backup);

        let server = TcpServerConnectivity::new(&server_config).await.unwrap();

        tokio::spawn(run_server(server, call.clone()));

        let mut client = TcpConnection::new(server_socket);

        let response = client.send_request(&call).await.unwrap();
        assert_eq!(response.inner(), &Response::BackupCreated);
        response.receive_blob().await.err().expect("No Blob expected");
    }
}