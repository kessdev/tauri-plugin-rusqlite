use crate::error::Error;
use rusqlite::ToSql;
use serde_json::Value as JsonValue;

pub type Result<T> = std::result::Result<T, Error>;
pub type Migrations = Vec<JsonValue>;
pub type SQLParameter = (String, Box<dyn ToSql>);
