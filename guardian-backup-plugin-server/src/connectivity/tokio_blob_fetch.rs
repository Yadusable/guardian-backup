use guardian_backup_domain::model::blobs::blob_fetch::BlobFetch;
use std::cmp::min;
use tokio::io::{AsyncRead, AsyncReadExt};

pub struct TokioBlobFetch<R: AsyncRead + Unpin> {
    reader: R,
    total_size: u64,
    read: u64,
}

impl<R: AsyncRead + Unpin> TokioBlobFetch<R> {
    pub fn new(reader: R, total_size: u64) -> Self {
        Self {
            reader,
            total_size,
            read: 0,
        }
    }
}

impl<R: AsyncRead + Unpin + Send> BlobFetch for TokioBlobFetch<R> {
    type Error = tokio::io::Error;

    fn remaining_len(&self) -> u64 {
        self.total_size - self.read
    }

    fn total_len(&self) -> u64 {
        self.total_size
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let buf_len = buf.len();
        let buf = &mut buf[..min(buf_len, self.remaining_len() as usize)];
        if buf.is_empty() {
            return Ok(0);
        }

        let just_read = self.reader.read(buf).await?;
        self.read += just_read as u64;
        Ok(just_read)
    }
}
