use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{
    ConnectionServerInterface, IncomingCall, UnhandledIncomingCall,
};
use guardian_backup_application::model::response::Response;
use guardian_backup_application::server_config::ServerConfig;
use guardian_backup_domain::helper::{CNone, COptional, CSome};
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::user_identifier::UserIdentifier;
use std::fmt::{Display, Formatter};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;

pub struct TcpServerConnectivity {
    server_socket: TcpListener,
}

impl TcpServerConnectivity {
    pub async fn new(config: &ServerConfig) -> std::io::Result<Self> {
        let listener = TcpListener::bind(config.bind_to).await?;

        Ok(Self {
            server_socket: listener,
        })
    }
}

impl ConnectionServerInterface for TcpServerConnectivity {
    type Error = TcpConnectivityError;
    type Call = IncomingTcpCall<CSome<Call>>;

    async fn receive_request(&mut self) -> Result<impl UnhandledIncomingCall, Self::Error> {
        let (incoming, client_address) = self.server_socket.accept().await?;
        log::info!("New incoming connection from {client_address}");

        let (rx, tx) = incoming.into_split();
        let mut rx = BufReader::new(rx);
        let tx = BufWriter::new(tx);

        let call_length = rx.read_u32().await?;
        let mut call_data = vec![0; call_length as usize];
        rx.read_exact(call_data.as_mut_slice()).await?;

        let call = ciborium::from_reader(call_data.as_slice())?;
        Ok(IncomingTcpCall {
            user: UserIdentifier::new(format!("TCP_{}", client_address).into()), //TODO actual user authentication
            rx,
            tx,
            call,
        })
    }
}

#[derive(Debug)]
pub struct IncomingTcpCall<CallHandled: COptional<Item = Call>> {
    rx: BufReader<OwnedReadHalf>,
    tx: BufWriter<OwnedWriteHalf>,
    call: CallHandled,
    user: UserIdentifier,
}

impl<CallHandled: COptional<Item = Call> + Send> IncomingTcpCall<CallHandled> {
    async fn send_response(
        &mut self,
        response: Response,
    ) -> Result<(), <Self as IncomingCall>::Error> {
        let mut response_data = Vec::new();
        ciborium::into_writer(&response, &mut response_data).expect("Vec can always grow");

        self.tx.write_u32(response_data.len() as u32).await?;
        self.tx.write_all(response_data.as_slice()).await?;

        Ok(())
    }

    async fn send_blob(
        &mut self,
        mut blob: impl BlobFetch,
    ) -> Result<(), <Self as IncomingCall>::Error> {
        debug_assert_eq!(blob.remaining_len(), blob.total_len());

        self.tx.write_u64(blob.remaining_len()).await?;

        let mut buf = [0; 1024];
        loop {
            let read = blob
                .read(buf.as_mut_slice())
                .await
                .map_err(|e| TcpConnectivityError::BlobFetch(format!("{e:?}").into()))?;
            if read == 0 {
                break;
            }

            self.tx.write_all(buf.split_at(read).0).await?
        }
        Ok(())
    }
}

impl<CallHandled: COptional<Item = Call> + Send> IncomingCall for IncomingTcpCall<CallHandled> {
    type Error = TcpConnectivityError;

    async fn answer(&mut self, response: Response) -> Result<(), Self::Error> {
        self.send_response(response).await?;
        self.tx.write_u64(0).await?; // Indicate a zero length blob
        self.tx.flush().await?;
        Ok(())
    }

    async fn answer_with_blob(
        &mut self,
        response: Response,
        blob_data: impl BlobFetch,
    ) -> Result<(), Self::Error> {
        self.send_response(response).await?;
        self.send_blob(blob_data).await?;
        self.tx.flush().await?;
        Ok(())
    }

    fn user(&self) -> &UserIdentifier {
        &self.user
    }

    async fn receive_blob(&mut self) -> Result<impl BlobFetch, Self::Error> {
        let blob_len = self.rx.read_u64().await?;
        if blob_len == 0 {
            return Err(TcpConnectivityError::NoBlob);
        }
        let fetch = TokioBlobFetch::new(&mut self.rx, blob_len);
        Ok(fetch)
    }
}

impl UnhandledIncomingCall for IncomingTcpCall<CSome<Call>> {
    fn into_inner(self) -> (Call, impl IncomingCall) {
        (
            self.call.0,
            IncomingTcpCall {
                rx: self.rx,
                tx: self.tx,
                call: CNone::default(),
                user: self.user,
            },
        )
    }

    fn inner(&self) -> &Call {
        &self.call.0
    }
}

#[derive(Debug)]
pub enum TcpConnectivityError {
    Io(std::io::Error),
    Ciborium(ciborium::de::Error<std::io::Error>),
    BlobFetch(Box<str>),
    NoBlob,
}

impl From<ciborium::de::Error<std::io::Error>> for TcpConnectivityError {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        Self::Ciborium(value)
    }
}

impl From<std::io::Error> for TcpConnectivityError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for TcpConnectivityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TcpConnectivityError::Io(inner) => write!(f, "{inner}"),
            TcpConnectivityError::Ciborium(inner) => write!(f, "{inner}"),
            TcpConnectivityError::BlobFetch(inner) => write!(f, "{inner}"),
            TcpConnectivityError::NoBlob => write!(f, "NoBLOB"),
        }
    }
}

impl std::error::Error for TcpConnectivityError {}

#[cfg(test)]
mod tests {
    use crate::connectivity::tcp_connectivity::TcpServerConnectivity;
    use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;
    use guardian_backup_application::in_memory_repositories::blob_repository::InMemoryBlobFetch;
    use guardian_backup_application::model::call::Call;
    use guardian_backup_application::model::connection_interface::ConnectionServerInterface;
    use guardian_backup_application::model::connection_interface::IncomingCall;
    use guardian_backup_application::model::connection_interface::UnhandledIncomingCall;
    use guardian_backup_application::model::response::Response;
    use guardian_backup_application::server_config::ServerConfig;
    use guardian_backup_domain::model::backup::backup::Backup;
    use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::tcp::OwnedWriteHalf;
    use tokio::net::TcpStream;

    async fn send_call(addr: SocketAddr) -> TcpStream {
        let backup = Backup::mock();
        let call = Call::CreateBackup(backup);

        let mut encoded = Vec::new();
        ciborium::into_writer(&call, &mut encoded).unwrap();

        let mut conn = TcpStream::connect(addr).await.unwrap();

        conn.write_u32(encoded.len() as u32).await.unwrap();
        conn.write_all(encoded.as_slice()).await.unwrap();

        conn
    }

    async fn send_blob(mut stream: OwnedWriteHalf) {
        let blob_data = [127; 4096];
        let mut blob = InMemoryBlobFetch::new(blob_data.into());

        stream.write_u64(blob.total_len()).await.unwrap();
        tokio::spawn(async move {
            let mut buf = [0; 1000];

            while blob.remaining_len() > 0 {
                let read = blob.read(&mut buf).await.unwrap();
                stream.write_all(buf.split_at(read).0).await.unwrap()
            }
            stream.flush().await.unwrap();
        });
    }

    async fn receive_response(stream: &mut TcpStream) -> Response {
        let response_len = stream.read_u32().await.unwrap();
        let mut response_buf = Vec::new();
        response_buf.resize(response_len as usize, 0);
        stream
            .read_exact(response_buf.as_mut_slice())
            .await
            .unwrap();

        let response = ciborium::de::from_reader(response_buf.as_slice()).unwrap();

        response
    }

    async fn receive_blob(mut stream: TcpStream) -> impl BlobFetch {
        let total_len = stream.read_u64().await.unwrap();
        let fetch = TokioBlobFetch::new(stream, total_len);

        fetch
    }

    #[tokio::test]
    async fn test_receive_request() {
        let server_config = ServerConfig::test_config();
        let mut server = TcpServerConnectivity::new(&server_config).await.unwrap();
        send_call(server_config.bind_to).await;

        let call = server.receive_request().await.unwrap();

        if let Call::CreateBackup(_) = call.inner() {
        } else {
            panic!("Expected Create Backup Call")
        }
    }

    #[tokio::test]
    async fn test_send_response() {
        let server_config = ServerConfig::test_config();
        let mut server = TcpServerConnectivity::new(&server_config).await.unwrap();
        let mut client = send_call(server_config.bind_to).await;

        let mut call = server.receive_request().await.unwrap();

        call.answer(Response::Successful).await.unwrap();

        let received_response = receive_response(&mut client).await;

        assert_eq!(received_response, Response::Successful);
    }

    #[tokio::test]
    async fn test_receive_blob() {
        let server_config = ServerConfig::test_config();
        let mut server = TcpServerConnectivity::new(&server_config).await.unwrap();
        let (_rx, tx) = send_call(server_config.bind_to).await.into_split();
        send_blob(tx).await;

        let mut call = server.receive_request().await.unwrap();
        let mut blob = call.receive_blob().await.unwrap();

        let blob_content = blob.read_to_eof().await.unwrap();

        assert_eq!(blob_content.as_ref(), &[127; 4096])
    }

    #[tokio::test]
    async fn test_send_response_with_blob() {
        let server_config = ServerConfig::test_config();
        let mut server = TcpServerConnectivity::new(&server_config).await.unwrap();
        let mut client = send_call(server_config.bind_to).await;

        let mut call = server.receive_request().await.unwrap();

        call.answer_with_blob(
            Response::Successful,
            InMemoryBlobFetch::new([127; 4096].into()),
        )
        .await
        .unwrap();

        let received_response = receive_response(&mut client).await;
        assert_eq!(received_response, Response::Successful);

        let mut received_blob = receive_blob(client).await;
        let received_blob_data = received_blob.read_to_eof().await.unwrap();
        assert_eq!(received_blob_data.as_ref(), &[127; 4096])
    }
}
