#![allow(async_fn_in_trait)]

use clap::Parser;
use guardian_backup_application::client_service::{ClientService, MainClientService};

mod cli;
mod connectivity;
mod tokio_file_service;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = cli::Cli::parse();
    //let mut client_service = MainClientService::new();
    //client_service.handle_command(cli.into()).await.unwrap();
}
