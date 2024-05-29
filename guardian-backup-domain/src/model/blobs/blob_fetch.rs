use std::fmt::Debug;

pub trait BlobFetch {
    type Error: Debug;
    
    fn remaining_len(&self) -> u64;
    fn total_len(&self) -> u64;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    async fn read_to_eof(&mut self) -> Result<Box<[u8]>, Self::Error> {
        let mut chunk = [0; 1024];
        let mut res = Vec::new();

        loop {
            let read = self.read(&mut chunk).await?;
            if read == 0 {
                break
            }

            res.extend_from_slice(chunk.split_at(read).0)
        }

        Ok(res.into())
    }
}
