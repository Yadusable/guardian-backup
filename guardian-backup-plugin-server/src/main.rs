#![allow(async_fn_in_trait)]
mod connectivity;


fn main() {
    
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use guardian_backup_application::model::connection_interface::ConnectionServerInterface;
    use guardian_backup_application::model::mocks::server_service_mock::MockServerService;
    use crate::connectivity::mock_connection::MockConnection;
    use guardian_backup_application::server_service::ServerService;

    #[tokio::test]
    pub async fn main_test() {
        let mut connection = MockConnection::default();
        let mut server_service = MockServerService::default();
        
        // non existent loop here
        let incoming_request = connection.receive_request().await.unwrap();
        server_service.handle_incoming_request(incoming_request).await.unwrap();
    }
}
