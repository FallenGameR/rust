// In real apps use anyhow crate for generic thread-safe errors
// p568
pub type ChatError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

