use crate::{
    error::Error,
    types::{Result, SQLParameter},
};
use rusqlite::{types::Value as SqliteValue, Statement};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub fn calculate_hash(text: &String) -> String {
    let digest = md5::compute(text.as_bytes());
    format!("{:x}", digest)
}

pub fn create_parameters(parameters: &HashMap<String, JsonValue>) -> Result<Vec<SQLParameter>> {
    let mut params = Vec::<SQLParameter>::new();
    for (name, value) in parameters {
        if value.is_null() {
            params.push((name.to_string(), Box::new(SqliteValue::Null)));
        } else if value.is_i64() {
            params.push((
                name.clone(),
                Box::new(SqliteValue::Integer(value.as_i64().unwrap())),
            ));
        } else if value.is_f64() {
            params.push((
                name.clone(),
                Box::new(SqliteValue::Real(value.as_f64().unwrap())),
            ));
        } else if value.is_string() {
            params.push((
                name.clone(),
                Box::new(SqliteValue::Text(value.as_str().unwrap().to_owned())),
            ));
        } else if value.is_array() {
            params.push((
                name.clone(),
                Box::new(SqliteValue::Blob(
                    value
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|element| element.as_i64().unwrap() as u8)
                        .collect::<Vec<u8>>(),
                )),
            ));
        } else {
            return Err(Error::Database(format!("({}: {})", name, value)));
        }
    }
    Ok(params)
}

pub fn get_column_names(statement: &Statement<'_>) -> Vec<String> {
    let mut column_names = Vec::<String>::new();
    for name in statement.column_names() {
        column_names.push(name.to_string());
    }
    column_names
}
