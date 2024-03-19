use crate::types::Result;
use commands::{migration::execute_migration, update::execute_update};
use error::Error;
use rusqlite::{Connection, OpenFlags};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Mutex};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};
use types::Migrations;

mod commands;
mod common;
mod error;
mod types;

#[derive(Default)]
struct ConfigState(Mutex<HashMap<String, Connection>>);

#[command]
async fn open_in_memory(state: State<'_, ConfigState>, name: String) -> Result<()> {
    let connection = Connection::open_in_memory()
        .map_err(|error| Error::OpeningConnection(error.to_string()))?;

    insert_connection(state, connection, name)
}

#[command]
async fn open_in_path(state: State<'_, ConfigState>, path: String) -> Result<()> {
    let connection = Connection::open_with_flags(path.clone(), OpenFlags::default())
        .map_err(|error| Error::OpeningConnection(error.to_string()))?;

    insert_connection(state, connection, path)
}

fn insert_connection(
    state: State<'_, ConfigState>,
    connection: Connection,
    name: String,
) -> Result<()> {
    let mut connections = state.0.lock().unwrap();
    let contains_key = connections.contains_key(&name);

    if !contains_key {
        connections.insert(name.clone(), connection);
    }

    Ok(())
}

#[command]
async fn migration(
    state: State<'_, ConfigState>,
    name: String,
    migrations: Migrations,
) -> Result<()> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    execute_migration(connection, migrations)
}

#[command]
async fn update(
    state: State<'_, ConfigState>,
    name: String,
    sql: String,
    parameters: HashMap<String, JsonValue>,
) -> Result<()> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    execute_update(connection, sql, parameters)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("rusqlite")
        .invoke_handler(tauri::generate_handler![
            open_in_memory,
            open_in_path,
            migration,
            update
        ])
        .setup(|app| {
            app.manage(ConfigState::default());
            Ok(())
        })
        .build()
}
