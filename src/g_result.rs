use std::error::Error;

type GError = Box<dyn Error + Send + Sync>;
pub type GResult<T> = Result<T, GError>;
