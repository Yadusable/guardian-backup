#![allow(async_fn_in_trait)]

use crate::blake_hash_service::BlakeHasher;
use crate::cbor_encoder_service::CborEncoderService;
use crate::tokio_file_service::TokioFileService;
use clap::Parser;
use guardian_backup_application::client_service::{ClientService, MainClientService};
use guardian_backup_application::in_memory_repositories::backup_repository::InMemoryBackupRepository;
use guardian_backup_application::in_memory_repositories::blob_repository::InMemoryBlobRepository;
use guardian_backup_domain::hash_service::HashService;
use guardian_backup_domain::model::user_identifier::UserIdentifier;

mod blake_hash_service;
mod cbor_encoder_service;
mod cli;
mod connectivity;
mod tokio_file;
mod tokio_file_service;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = cli::Cli::parse();
    let mut client_service: MainClientService<_, _, CborEncoderService, TokioFileService> =
        MainClientService::new(
            UserIdentifier::new("TestUser".into()),
            InMemoryBackupRepository::new(),
            InMemoryBlobRepository::new(),
            HashService::new(vec![&BlakeHasher()]),
        );
    client_service.handle_command(cli.into()).await.unwrap();
}
