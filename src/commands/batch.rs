use crate::{error::Error, types::Result};
use rusqlite::Connection;

pub fn execute_batch(connection: &Connection, batch_sql: String) -> Result<()> {
    connection
        .execute_batch(&batch_sql)
        .map_err(|error| Error::Database(error.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_batch_test() {
        let connection = Connection::open_in_memory().unwrap();
        let batch_sql = r#"
            CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
            INSERT INTO users (name) VALUES ('Alice');
            "#
        .to_string();
        execute_batch(&connection, batch_sql).unwrap();

        let select_sql = "SELECT * FROM users";
        let mut statement = connection.prepare(select_sql).unwrap();
        let mut rows = statement.query([]).unwrap();

        let first_row = rows.next().unwrap().unwrap();

        assert_eq!(first_row.get::<_, i64>(0).unwrap(), 1);
        assert_eq!(first_row.get::<_, String>(1).unwrap(), "Alice");

        assert!(rows.next().unwrap().is_none());
    }

    #[test]
    fn execute_batch_drop_table_test() {
        let connection = Connection::open_in_memory().unwrap();
        let create_table_sql = r#"
            CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
            INSERT INTO users (name) VALUES ('Alice');
            "#
        .to_string();
        let result = execute_batch(&connection, create_table_sql);
        assert!(result.is_ok());

        let drop_table_sql = "DROP TABLE users;".to_string();
        let result = execute_batch(&connection, drop_table_sql);
        assert!(result.is_ok());
    }
}
