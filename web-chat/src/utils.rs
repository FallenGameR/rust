// In real apps use anyhow crate for generic thread-safe errors
// p568
pub type AppError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type AppResult<T> = Result<T, AppError>;

