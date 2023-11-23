use crate::model::password_hash::PasswordHash;

pub enum Credential {
    Password {
        hash: PasswordHash,
    }
}