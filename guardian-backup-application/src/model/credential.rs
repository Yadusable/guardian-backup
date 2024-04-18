use crate::model::password_hash_algorithms::PasswordHashAlg;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq)]
pub enum Credential {
    Password {
        algorithm: PasswordHashAlg,
        hash: Box<[u8]>,
    },
}

impl Debug for Credential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Credential::")?;
        match &self {
            Credential::Password { hash, algorithm } => {
                write!(
                    f,
                    "Password {{hash: {:?}...{:?},",
                    &hash[0..4],
                    &hash[(hash.len() - 4)..hash.len()]
                )?;
                Debug::fmt(algorithm, f)?;
                write!(f, "}}")
            }
        }
    }
}
