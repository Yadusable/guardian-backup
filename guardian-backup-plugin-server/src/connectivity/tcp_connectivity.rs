use std::fmt::{Display, Formatter};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{ConnectionServerInterface, IncomingCall, UnhandledIncomingCall};
use guardian_backup_application::server_config::ServerConfig;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::helper::{CNone, COptional, CSome};
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::user_identifier::UserIdentifier;

pub struct TcpServerConnectivity {
    server_socket: TcpListener,
}

impl TcpServerConnectivity {
    pub async fn new(config: &ServerConfig) -> std::io::Result<Self> {
        let listener = TcpListener::bind(config.bind_to).await?;

        Ok(Self {
            server_socket: listener
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
        let mut tx = BufWriter::new(tx);

        let call_length = rx.read_u32().await?;
        let mut call_data = Vec::new();
        call_data.resize(call_length as usize, 0);
        rx.read_exact(call_data.as_mut_slice()).await?;

        let call = ciborium::from_reader(call_data.as_slice())?;
        Ok(IncomingTcpCall{
            user: UserIdentifier::new(format!("TCP_{}", client_address).into()), //TODO actual user authentication
            rx,
            tx,
            call,
        })
    }
}

#[derive(Debug)]
pub struct IncomingTcpCall<CallHandled: COptional<Item=Call>> {
    rx: BufReader<OwnedReadHalf>,
    tx: BufWriter<OwnedWriteHalf>,
    call: CallHandled,
    user: UserIdentifier,
}

impl<CallHandled: COptional<Item=Call>> IncomingTcpCall<CallHandled> {
    async fn send_response(&mut self, response: Response) -> Result<(), <Self as IncomingCall>::Error>{
        let mut response_data = Vec::new();
        ciborium::into_writer(&response, &mut response_data).expect("Vec can always grow");

        self.tx.write_u32(response_data.len() as u32).await?;
        self.tx.write_all(response_data.as_slice()).await?;

        Ok(())
    }

    async fn send_blob(&mut self, mut blob: impl BlobFetch) -> Result<(), <Self as IncomingCall>::Error> {
        let mut buf = [0; 1024];
        loop {
            let read = blob.read(buf.as_mut_slice()).await.map_err(|e| TcpConnectivityError::BlobFetch(format!("{e:?}").into()))?;
            if read == 0 {
                break;
            }

            self.tx.write_all(buf.split_at(read).0).await?
        }
        Ok(())
    }
}

impl<CallHandled: COptional<Item=Call>> IncomingCall for IncomingTcpCall<CallHandled> {
    type Error = TcpConnectivityError;

    async fn answer(mut self, response: Response) -> Result<(), Self::Error> {
        self.send_response(response).await?;
        self.tx.flush().await?;
        Ok(())
    }

    async fn answer_with_blob(mut self, response: Response, blob_data: impl BlobFetch) -> Result<(), Self::Error> {
        self.send_response(response).await?;
        self.send_blob(blob_data).await?;
        self.tx.flush().await?;
        Ok(())
    }

    fn user(&self) -> &UserIdentifier {
        &self.user
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
            }
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
        }
    }
}

impl std::error::Error for TcpConnectivityError {}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use guardian_backup_application::model::call::Call;
    use guardian_backup_application::model::connection_interface::ConnectionServerInterface;
    use guardian_backup_application::server_config::ServerConfig;
    use guardian_backup_domain::model::backup::backup::Backup;
    use guardian_backup_application::model::connection_interface::UnhandledIncomingCall;
    use guardian_backup_application::model::response::Response;
    use guardian_backup_application::model::connection_interface::IncomingCall;
    use crate::connectivity::tcp_connectivity::TcpServerConnectivity;

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

    async fn receive_response(stream: &mut TcpStream) -> Response {
        let response_len = stream.read_u32().await.unwrap();
        let mut response_buf = Vec::new();
        response_buf.resize(response_len as usize, 0);
        stream.read_exact(response_buf.as_mut_slice()).await.unwrap();

        let response = ciborium::de::from_reader(response_buf.as_slice()).unwrap();

        response
    }

    #[tokio::test]
    async fn test_receive_request() {
        let server_config = ServerConfig::test_config();
        let mut server = TcpServerConnectivity::new(&server_config).await.unwrap();
        send_call(server_config.bind_to).await;

        let call = server.receive_request().await.unwrap();

        assert_eq!(call.inner(), &Call::CreateBackup(Backup::mock()))
    }

    #[tokio::test]
    async fn test_send_response() {
        let server_config = ServerConfig::test_config();
        let mut server = TcpServerConnectivity::new(&server_config).await.unwrap();
        let mut client = send_call(server_config.bind_to).await;

        let call = server.receive_request().await.unwrap();

        call.answer(Response::BackupCreated).await.unwrap();

        let received_response = receive_response(&mut client).await;

        assert_eq!(received_response, Response::BackupCreated);
    }

}