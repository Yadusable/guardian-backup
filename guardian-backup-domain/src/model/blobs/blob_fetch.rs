use std::future::Future;

pub trait BlobFetch: Send {
    type Error: std::error::Error + 'static;

    fn remaining_len(&self) -> u64;
    fn total_len(&self) -> u64;

    fn read(&mut self, buf: &mut [u8]) -> impl Future<Output = Result<usize, Self::Error>> + Send;

    fn read_to_eof(&mut self) -> impl Future<Output = Result<Box<[u8]>, Self::Error>> + Send {
        async {
            let mut chunk = [0; 1024];
            let mut res = Vec::new();

            loop {
                let read = self.read(&mut chunk).await?;
                if read == 0 {
                    break;
                }

                res.extend_from_slice(chunk.split_at(read).0)
            }

            Ok(res.into())
        }
    }
}
