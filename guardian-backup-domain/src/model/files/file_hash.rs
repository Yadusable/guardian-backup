#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FileHash {
    Blake3 { hash: [u8; 64] },
}
