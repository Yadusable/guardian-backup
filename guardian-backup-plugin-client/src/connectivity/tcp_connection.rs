use std::net::SocketAddr;
use guardian_backup_application::model::call::Call;
use guardian_backup_application::model::connection_interface::ConnectionClientInterface;
use guardian_backup_application::model::response::Response;

pub struct TcpConnection {

}

impl TcpConnection {
    pub fn new(addr: SocketAddr) -> Self {
        todo!()
    }
}

impl ConnectionClientInterface for TcpConnection {
    type Error = ();

    async fn send_request(&mut self, command: Call) -> Result<Response, Self::Error> {
        todo!()
    }
}