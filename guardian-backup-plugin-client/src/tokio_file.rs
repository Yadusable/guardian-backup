use crate::connectivity::tokio_blob_fetch::TokioBlobFetch;
use guardian_backup_application::file_service::File;
use guardian_backup_domain::hash_service::Hasher;
use guardian_backup_domain::hash_service::PendingHashB;
use guardian_backup_domain::hash_service::PendingHashExt;
use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use guardian_backup_domain::model::files::file_hash::FileHash;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

pub struct TokioFile {
    path: PathBuf,
}

impl File for TokioFile {
    type Error = tokio::io::Error;

    async fn get_hash<H: Hasher>(&self, hasher: H) -> Result<FileHash, Self::Error> {
        let file_blob = self.get_as_blob().await?;
        let mut hash = hasher.create_hash();
        hash.update_blob(file_blob);
        Ok(hash.finalize())
    }

    async fn get_size(&self) -> Result<u64, Self::Error> {
        Ok(tokio::fs::metadata(self.path.as_path()).await?.len())
    }

    async fn get_last_modified(&self) -> Result<u64, Self::Error> {
        Ok(tokio::fs::metadata(self.path.as_path())
            .await?
            .modified()?
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64)
    }

    async fn get_as_blob(&self) -> Result<impl BlobFetch, Self::Error> {
        let file = tokio::fs::File::open(self.path.as_path()).await?;
        let file_len = file.metadata().await?.len();
        Ok(TokioBlobFetch::new(file, file_len))
    }
}

impl TokioFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
