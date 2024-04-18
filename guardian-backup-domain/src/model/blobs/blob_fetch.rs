pub trait BlobFetch {
    type Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    async fn read_to_eof(self) -> Result<Box<[u8]>, Self::Error>;
}
