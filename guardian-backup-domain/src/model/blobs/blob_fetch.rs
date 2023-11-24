use crate::model::error::AsyncResult;

pub trait BlobFetch {
    fn read(&mut self, buf: &mut [u8]) -> AsyncResult<usize>;

    fn read_to_eof(&mut self) -> AsyncResult<Vec<u8>> {
        todo!()
    }
}
