use serde::{Deserialize, Serialize};
use std::io::Read;

pub trait EncodingService {
    type Error: std::error::Error + 'static;

    fn decode<T: for<'d> Deserialize<'d>>(data: impl Read) -> Result<T, Self::Error>;
    fn encode<T: Serialize>(payload: T) -> Vec<u8>;
}
