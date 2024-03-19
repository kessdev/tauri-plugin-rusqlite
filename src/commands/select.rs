use std::collections::HashMap;

use rusqlite::{types::Value as SqliteValue, Connection, ToSql};

use serde_json::{Number, Value as JsonValue};

use crate::common::{create_parameters, get_column_names};
use crate::error::Error;
use crate::types::{Result, ResultElement, ResultList};

pub fn execute_select(
    connection: &Connection,
    sql: String,
    parameters: HashMap<String, JsonValue>,
) -> Result<ResultList> {
    let sql_parameters = create_parameters(&parameters)?;
    let params = sql_parameters
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_ref()))
        .collect::<Vec<(&str, &dyn ToSql)>>();

    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| Error::Database(error.to_string()))?;

    let column_names = get_column_names(&statement);

    let mut result = ResultList::new();
    let mut rows = statement
        .query(params.as_slice())
        .map_err(|error| Error::Database(error.to_string()))?;

    while let Some(row) = rows.next().unwrap() {
        let mut map = ResultElement::new();
        for (index, name) in column_names.iter().enumerate() {
            let row_value = row
                .get_ref(index)
                .map_err(|error| Error::Database(error.to_string()))?;
            let value = match SqliteValue::from(row_value) {
                SqliteValue::Null => JsonValue::Null,
                SqliteValue::Integer(value) => JsonValue::Number(value.into()),
                SqliteValue::Real(value) => JsonValue::Number(Number::from_f64(value).unwrap()),
                SqliteValue::Text(value) => JsonValue::String(value),
                SqliteValue::Blob(value) => JsonValue::Array(
                    value
                        .iter()
                        .map(|byte| JsonValue::Number((*byte).into()))
                        .collect(),
                ),
            };
            map.insert(name.clone(), value);
        }
        result.push(map);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_query_test() {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB);
            INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (1, 1.1, 'test1', x'010203');
            INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (null, null, null, null);"
        ).unwrap();

        let sql = "SELECT * FROM test WHERE id = :id";

        let mut parameters = HashMap::new();
        parameters.insert(":id".to_string(), JsonValue::Number(1.into()));
        let result = execute_select(&connection, sql.to_string(), parameters).unwrap();

        assert_eq!(
            result[0].get("integer_value").unwrap(),
            &JsonValue::Number(1.into())
        );
        assert_eq!(
            result[0].get("real_value").unwrap(),
            &JsonValue::Number(Number::from_f64(1.1).unwrap())
        );
        assert_eq!(
            result[0].get("text_value").unwrap(),
            &JsonValue::String("test1".to_string())
        );
        assert_eq!(
            result[0].get("blob_value").unwrap(),
            &JsonValue::Array(vec![
                JsonValue::Number(1.into()),
                JsonValue::Number(2.into()),
                JsonValue::Number(3.into())
            ])
        );

        let mut parameters = HashMap::new();
        parameters.insert(":id".to_string(), JsonValue::Number(2.into()));
        let result = execute_select(&connection, sql.to_string(), parameters).unwrap();

        assert_eq!(result[0].get("integer_value").unwrap(), &JsonValue::Null);
        assert_eq!(result[0].get("real_value").unwrap(), &JsonValue::Null);
        assert_eq!(result[0].get("text_value").unwrap(), &JsonValue::Null);
        assert_eq!(result[0].get("blob_value").unwrap(), &JsonValue::Null);
    }

    #[test]
    fn execute_query_empty_parameters_test() {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB);
            INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (1, 1.1, 'test1', x'010203');
            INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (2, 2.2, 'test2', x'040506');"
        ).unwrap();

        let sql = "SELECT count(*) as rows FROM test";

        let parameters = HashMap::new();
        let result = execute_select(&connection, sql.to_string(), parameters).unwrap();

        assert_eq!(result[0].get("rows").unwrap(), &JsonValue::Number(2.into()));
    }
}
