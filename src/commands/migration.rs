use rusqlite::Connection;

use crate::common::calculate_hash;
use crate::error::Error;
use crate::types::{Migrations, Result};

pub fn execute_migration(connection: &Connection, migrations: Migrations) -> Result<()> {
    let migration_table_sql = "CREATE TABLE IF NOT EXISTS migrations_history (id INTEGER PRIMARY KEY, name TEXT NOT NULL, hash TEXT NOT NULL)";
    connection
        .execute_batch(migration_table_sql)
        .or_else(|error| Err(Error::DatabaseError(error.to_string())))?;

    let mut statement = connection
        .prepare(&format!(
            "SELECT name, hash FROM migrations_history ORDER BY id"
        ))
        .or_else(|error| Err(Error::DatabaseError(error.to_string())))?;

    let mut migrations_iterator = migrations.iter();
    let mut rows = statement
        .query([])
        .or_else(|error| Err(Error::DatabaseError(error.to_string())))?;

    while let Some(row) = rows.next().unwrap() {
        let name_value = row.get_ref(0).unwrap();
        let hash_value = row.get_ref(1).unwrap();

        let migration_option = migrations_iterator.next();

        match migration_option {
            Some(migration) => {
                let name = migration.get("name").unwrap().as_str().unwrap();
                let sql = migration.get("sql").unwrap().as_str().unwrap();
                let hash = calculate_hash(&sql.to_string());
                if name_value.as_str().unwrap() != name || hash_value.as_str().unwrap() != hash {
                    return Err(Error::MigrationError(format!(
                        "The migration {} has been modified",
                        name
                    )));
                }
            }
            None => {
                return Err(Error::MigrationError(
                    "The migration list has been modified".to_string(),
                ));
            }
        }
    }

    while let Some(migration) = migrations_iterator.next() {
        let name = migration.get("name").unwrap().as_str().unwrap();
        let sql = migration.get("sql").unwrap().as_str().unwrap();
        let hash = calculate_hash(&sql.to_string());

        connection.execute_batch(sql).or_else(|error| {
            Err(Error::MigrationError(format!(
                "Error executing migration: {}. {}",
                name,
                error.to_string()
            )))
        })?;

        let mut statement = connection
            .prepare("INSERT INTO migrations_history (name, hash) VALUES (:name, :hash)")
            .or_else(|error| {
                Err(Error::MigrationError(format!(
                    "Error preparing migration: {}. {}",
                    name,
                    error.to_string()
                )))
            })?;

        statement
            .execute(&[(":name", name), (":hash", &&hash)])
            .or_else(|error| Err(Error::MigrationError(error.to_string())))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::Value as JsonValue;

    use super::*;

    #[test]
    fn execute_migration_test() {
        let connection = Connection::open_in_memory().unwrap();
        let mut migrations = Migrations::new();

        let create_table = "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB)";
        let mut map = Map::new();
        map.insert(
            "name".to_string(),
            JsonValue::String("create_test_table".to_string()),
        );
        map.insert(
            "sql".to_string(),
            JsonValue::String(create_table.to_string()),
        );
        migrations.push(JsonValue::Object(map));

        let insert_data = "INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (1, 1.1, 'test1', x'010203')";
        let mut map = Map::new();
        map.insert(
            "name".to_string(),
            JsonValue::String("insert_test_data".to_string()),
        );
        map.insert(
            "sql".to_string(),
            JsonValue::String(insert_data.to_string()),
        );
        migrations.push(JsonValue::Object(map));

        execute_migration(&connection, migrations).unwrap();

        let count_sql = "SELECT count(*) FROM migrations_history";

        let mut statement = connection.prepare(count_sql).unwrap();
        let mut rows = statement.query([]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            let count = row.get_ref(0).unwrap();
            assert_eq!(count.as_i64().unwrap(), 2);
        } else {
            assert!(false);
        }

        let sql = "SELECT * FROM migrations_history WHERE id = ?1";

        let mut statement = connection.prepare(sql).unwrap();
        let mut rows = statement.query([1]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            let name = row.get_ref(1).unwrap();
            let hash = row.get_ref(2).unwrap();
            assert_eq!(name.as_str().unwrap(), "create_test_table");
            assert_eq!(
                hash.as_str().unwrap(),
                calculate_hash(&create_table.to_string())
            );
        } else {
            assert!(false);
        }

        let mut statement = connection.prepare(sql).unwrap();
        let mut rows = statement.query([2]).unwrap();
        if let Some(row) = rows.next().unwrap() {
            let name = row.get_ref(1).unwrap();
            let hash = row.get_ref(2).unwrap();
            assert_eq!(name.as_str().unwrap(), "insert_test_data");
            assert_eq!(
                hash.as_str().unwrap(),
                calculate_hash(&insert_data.to_string())
            );
        } else {
            assert!(false);
        }
    }
}
