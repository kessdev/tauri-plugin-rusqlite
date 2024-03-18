use crate::error::Error;
use serde_json::Value as JsonValue;
use rusqlite::ToSql;

pub type Result<T> = std::result::Result<T, Error>;
pub type Migrations = Vec<JsonValue>;
pub type SQLParameter = (String, Box<dyn ToSql>);