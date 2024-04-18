use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum CompressionAlgorithm {
    Plain,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompressedContainer {
    compressor: CompressionAlgorithm,
    inner: Box<[u8]>,
}
