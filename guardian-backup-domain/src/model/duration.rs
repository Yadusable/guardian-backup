pub enum Duration {
    Infinite,
    Limited {
        milliseconds: u64
    },
}