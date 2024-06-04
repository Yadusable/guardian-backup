#![allow(async_fn_in_trait)]

use guardian_backup_application::in_memory_repositories::backup_repository::InMemoryBackupRepository;
use guardian_backup_application::in_memory_repositories::blob_repository::InMemoryBlobRepository;
use guardian_backup_application::model::connection_interface::ConnectionServerInterface;
use guardian_backup_application::server_config::ServerConfig;
use guardian_backup_application::server_service::{MainServerService, ServerService};
use guardian_backup_plugin_server::connectivity::tcp_connectivity::TcpServerConnectivity;

pub mod connectivity;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let backup_repository = InMemoryBackupRepository::new();
    let blob_repository = InMemoryBlobRepository::new();
    let server_config = ServerConfig::default();

    let mut service = MainServerService::new(backup_repository, blob_repository);

    let mut connection = TcpServerConnectivity::new(&server_config).await.unwrap();

    loop {
        let request = connection.receive_request().await.unwrap();
        service.handle_incoming_request(request).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::connectivity::mock_connection::MockConnection;
    use guardian_backup_application::model::connection_interface::ConnectionServerInterface;
    use guardian_backup_application::model::mocks::server_service_mock::MockServerService;
    use guardian_backup_application::server_service::ServerService;

    #[tokio::test]
    pub async fn main_test() {
        let mut connection = MockConnection::default();
        let mut server_service = MockServerService::default();

        // non existent loop here
        let incoming_request = connection.receive_request().await.unwrap();
        server_service
            .handle_incoming_request(incoming_request)
            .await
            .unwrap();
    }
}
