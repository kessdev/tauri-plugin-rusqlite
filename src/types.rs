use crate::error::Error;
use rusqlite::ToSql;
use serde_json::{Map, Value as JsonValue};

pub type Result<T> = std::result::Result<T, Error>;
pub type Migrations = Vec<JsonValue>;
pub type SQLParameter = (String, Box<dyn ToSql>);
pub type ResultElement = Map<String, JsonValue>;
pub type ResultList = Vec<ResultElement>;
