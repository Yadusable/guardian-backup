pub enum FileHash {
    Blake3 {
        hash: [u8; 64]
    }
}