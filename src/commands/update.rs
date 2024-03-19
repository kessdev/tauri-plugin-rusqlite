use std::collections::HashMap;

use rusqlite::{Connection, ToSql};

use serde_json::Value as JsonValue;

use crate::common::create_parameters;
use crate::error::Error;
use crate::types::Result;

pub fn execute_update(
    connection: &Connection,
    sql: String,
    parameters: HashMap<String, JsonValue>,
) -> Result<()> {
    let sql_parameters = create_parameters(&parameters)?;
    let params = sql_parameters
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_ref()))
        .collect::<Vec<(&str, &dyn ToSql)>>();

    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| Error::Database(error.to_string()))?;

    statement
        .execute(params.as_slice())
        .map_err(|error| Error::Database(error.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::Number;

    use super::*;

    #[test]
    fn execute_update_insert_data_test() {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB);"
        ).unwrap();

        let sql = "INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (:integer_value, :real_value, :text_value, :blob_value)";

        let mut parameters = HashMap::new();
        parameters.insert(":integer_value".to_string(), JsonValue::Number(1.into()));
        parameters.insert(
            ":real_value".to_string(),
            JsonValue::Number(Number::from_f64(1.1).unwrap()),
        );
        parameters.insert(
            ":text_value".to_string(),
            JsonValue::String("test1".to_string()),
        );
        parameters.insert(
            ":blob_value".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(1.into()),
                JsonValue::Number(2.into()),
                JsonValue::Number(3.into()),
            ]),
        );
        execute_update(&connection, sql.to_string(), parameters).unwrap();

        let sql = "SELECT * FROM test WHERE id = :id";

        let mut statement = connection.prepare(sql).unwrap();
        let mut rows = statement.query([1]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            assert_eq!(row.get_ref(1).unwrap().as_i64().unwrap(), 1);
            assert_eq!(row.get_ref(2).unwrap().as_f64().unwrap(), 1.1);
            assert_eq!(row.get_ref(3).unwrap().as_str().unwrap(), "test1");
            assert_eq!(row.get_ref(4).unwrap().as_blob().unwrap(), &[1, 2, 3]);
        } else {
            panic!();
        }
    }

    #[test]
    fn execute_update_update_data_test() {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB);
            INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (1, 1.1, 'test1', x'010203');"
        ).unwrap();

        let sql = "UPDATE test SET integer_value = :integer_value, real_value = :real_value, text_value = :text_value, blob_value = :blob_value WHERE id = :id";

        let mut parameters = HashMap::new();
        parameters.insert(":id".to_string(), JsonValue::Number(1.into()));
        parameters.insert(":integer_value".to_string(), JsonValue::Number(3.into()));
        parameters.insert(
            ":real_value".to_string(),
            JsonValue::Number(Number::from_f64(3.3).unwrap()),
        );
        parameters.insert(
            ":text_value".to_string(),
            JsonValue::String("test3".to_string()),
        );
        parameters.insert(
            ":blob_value".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(7.into()),
                JsonValue::Number(8.into()),
                JsonValue::Number(9.into()),
            ]),
        );
        execute_update(&connection, sql.to_string(), parameters).unwrap();

        let sql = "SELECT * FROM test WHERE id = :id";

        let mut statement = connection.prepare(sql).unwrap();
        let mut rows = statement.query([1]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            assert_eq!(row.get_ref(1).unwrap().as_i64().unwrap(), 3);
            assert_eq!(row.get_ref(2).unwrap().as_f64().unwrap(), 3.3);
            assert_eq!(row.get_ref(3).unwrap().as_str().unwrap(), "test3");
            assert_eq!(row.get_ref(4).unwrap().as_blob().unwrap(), &[7, 8, 9]);
        } else {
            panic!();
        }
    }
}
