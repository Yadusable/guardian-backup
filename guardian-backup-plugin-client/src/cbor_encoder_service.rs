use guardian_backup_application::encoding_service::EncodingService;
use std::io::Read;

pub struct CborEncoderService {}

impl EncodingService for CborEncoderService {
    type Error = ciborium::de::Error<std::io::Error>;

    fn decode<T: for<'d> serde::de::Deserialize<'d>>(data: impl Read) -> Result<T, Self::Error> {
        ciborium::from_reader(data)
    }

    fn encode<T: serde::ser::Serialize>(payload: T) -> Vec<u8> {
        let mut encoded = vec![];
        ciborium::into_writer(&payload, &mut encoded).unwrap();
        encoded
    }
}

impl CborEncoderService {
    pub fn new() -> Self {
        CborEncoderService {}
    }
}
