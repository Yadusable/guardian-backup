#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PasswordHashAlg {
    Argon2id {
        hash: [u8; 32],
        salt: [u8; 16],
        parallelism: u32,
        memory_cost: u32,
        iterations: u32,
        version: u32,
    },
}
