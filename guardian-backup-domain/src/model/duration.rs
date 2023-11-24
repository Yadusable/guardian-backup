#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Duration {
    Infinite,
    Limited { milliseconds: u64 },
}
