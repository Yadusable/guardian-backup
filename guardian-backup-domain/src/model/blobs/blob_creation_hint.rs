#[derive(Debug)]
pub struct BlobCreationHint {
    pub should_compress: bool,
}

impl Default for BlobCreationHint {
    fn default() -> Self {
        BlobCreationHint {
            should_compress: true,
        }
    }
}
