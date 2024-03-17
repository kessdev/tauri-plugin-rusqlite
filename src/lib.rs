use commands::migration::execute_migration;
use error::Error;
use rusqlite::{Connection, OpenFlags};
use tauri::{
  command,
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, State
};
use types::Migrations;
use std::{collections::HashMap, sync::Mutex};
use crate::types::Result;

mod common;
mod error;
mod types;
mod commands;

#[derive(Default)]
struct ConfigState(Mutex<HashMap<String, Connection>>);

#[command]
async fn open_in_memory(state: State<'_, ConfigState>, name: String) -> Result<()> {
    let connection = Connection::open_in_memory()
        .or_else(|error| Err(Error::OpeningConnectionError(error.to_string())))?;

    insert_connection(state, connection, name)
}

#[command]
async fn open_in_path(state: State<'_, ConfigState>, path: String) -> Result<()> {
    let connection = Connection::open_with_flags(path.clone(), OpenFlags::default())
        .or_else(|error| Err(Error::OpeningConnectionError(error.to_string())))?;

    insert_connection(state, connection, path)
}

fn insert_connection(state: State<'_, ConfigState>, connection: Connection, name: String) -> Result<()> {
  let mut connections = state.0.lock().unwrap();
  let contains_key = connections.contains_key(&name);

  if !contains_key {
      connections.insert(name.clone(), connection);
  }

  Ok(())
}

#[command]
async fn migration(state: State<'_, ConfigState>, name: String, migrations: Migrations) -> Result<()> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::ConnectionError()),
    };

    execute_migration(connection, migrations)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("rusqlite")
    .invoke_handler(tauri::generate_handler![
      open_in_memory,
      open_in_path,
      migration
    ])
    .setup(|app| {
      app.manage(ConfigState::default());
      Ok(())
    })
    .build()
}
