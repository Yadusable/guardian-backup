#![allow(async_fn_in_trait)]

use clap::Parser;
use guardian_backup_application::client_service::{ClientService, MainClientService};

mod cli;
mod connectivity;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let mut client_service = MainClientService::new();
    client_service.handle_command(cli.into()).await.unwrap();
}
