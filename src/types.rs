use crate::error::Error;
use serde_json::Value as JsonValue;

pub type Result<T> = std::result::Result<T, Error>;
pub type Migrations = Vec<JsonValue>;