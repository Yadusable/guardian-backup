use crate::model::user_identifier::UserIdentifier;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::pin::Pin;

pub type Result<T> = std::result::Result<T, GuardianError>;
pub type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T>>>>;

#[derive(Debug)]
pub enum GuardianError {
    UserNotFound { identifier: UserIdentifier },
    InternalError { message: Cow<'static, str> },
}

impl Display for GuardianError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardianError::UserNotFound { identifier } => {
                write!(f, "UserNotFound: User '{identifier}' was not found")
            }
            GuardianError::InternalError { message } => {
                write!(f, "InternalError: {message}")
            }
        }
    }
}

impl Error for GuardianError {}
