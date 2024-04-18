use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpListener;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::{ConnectionServerInterface, IncomingCall};
use guardian_backup_application::server_config::ServerConfig;
use serde::{Deserialize, Serialize};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use guardian_backup_application::model::response::Response;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;

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
    type Call = IncomingTcpCall;

    async fn receive_request(&mut self) -> Result<Self::Call, Self::Error> {
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
            rx,
            tx,
            call,
        })
    }
}

#[derive(Debug)]
pub struct IncomingTcpCall {
    rx: BufReader<OwnedReadHalf>,
    tx: BufWriter<OwnedWriteHalf>,
    call: Call,
}

impl IncomingCall for IncomingTcpCall {
    type Error = TcpConnectivityError;

    async fn answer<T: BlobFetch>(mut self, response: Response, blob_data: Option<T>) -> Result<(), Self::Error> {
        let mut response_data = Vec::new();
        ciborium::into_writer(&response, response_data.as_mut_slice()).expect("Vec can always grow");

        self.tx.write_u32(response_data.len() as u32).await?;
        self.tx.write_all(response_data.as_slice()).await?;

        if let Some(mut blob_data) = blob_data {
            let mut buf = [0; 1024];
            loop {
                let read = blob_data.read(buf.as_mut_slice()).await.map_err(|e| TcpConnectivityError::BlobFetch(format!("{e:?}").into()))?;
                if read == 0 {
                    break;
                }

                self.tx.write_all(buf.split_at(read).0).await?
            }
        }

        Ok(())
    }

    fn inner(&self) -> &Call {
        &self.call
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