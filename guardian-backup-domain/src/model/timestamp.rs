#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Timestamp {
    milliseconds_since_epoch: u64,
}
