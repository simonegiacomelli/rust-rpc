use std::error::Error;
use serde::Serialize;

type GError = Box<dyn Error + Send + Sync>;
pub type GResult<T> = Result<T, GError>;
