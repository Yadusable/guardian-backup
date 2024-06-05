use crate::model::client_model::{ClientBackupCommand, ClientCommand, ClientSubcommand};
use std::convert::Infallible;

pub trait ClientService {
    type Error: std::error::Error;

    async fn handle_command(&mut self, command: ClientCommand) -> Result<(), Self::Error>;
}

pub struct MainClientService {}

impl MainClientService {
    pub fn new() -> Self {
        todo!()
    }
}

impl ClientService for MainClientService {
    type Error = Infallible;

    async fn handle_command(&mut self, command: ClientCommand) -> Result<(), Self::Error> {
        match command.subcommand {
            ClientSubcommand::Server { .. } => {
                unimplemented!()
            }
            ClientSubcommand::Backup(inner) => match inner {
                ClientBackupCommand::Auto { .. } => {
                    todo!()
                }
                ClientBackupCommand::Create { .. } => {
                    todo!()
                }
                ClientBackupCommand::Restore { .. } => {
                    todo!()
                }
            },
        }
    }
}
