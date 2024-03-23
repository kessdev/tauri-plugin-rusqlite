# Tauri Plugin Rusqlite

This plugin enables access to an SQLite database within Tauri applications. It is inspired by the [tauri-plugin-sqlite](https://github.com/lzdyes/tauri-plugin-sqlite) plugin but is based on [rusqlite](https://github.com/rusqlite/rusqlite).

## Example

- Full example at: <https://github.com/kessdev/tauri-plugin-rusqlite/tree/main/examples/rusqlite-demo>
- Execute the following command in your terminal:

``` bash
npm run tauri dev
```

## Installation

### Rust

- Enter the `src-tauri` directory in your project's structure.

- Execute the following command to add tauri-plugin-rusqlite to your project's dependencies:

``` bash
cargo add tauri-plugin-rusqlite
```

- Open the `main.rs` file located in the `src-tauri/src` directory of your project. Modify the `main` function to include the plugin initialization code.

``` rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_rusqlite::init()) // add this
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Webview

- Navigate to the root directory of your source code.

- Execute the following command in your terminal:

``` bash
npm i tauri-plugin-rusqlite-api
```

## Usage

### Import plugin

``` ts
import Rusqlite from 'tauri-plugin-rusqlite-api'
```

### Open database

``` ts
const database = await Rusqlite.openInMemory("test.db");
```
or
``` ts
const database = await Rusqlite.openInPath("./folder/test.db");
```

### Init database

``` ts
let scripts = [{
    name: "create_table", 
    sql: "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB); CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);"
}];
await database.migration(scripts);
```
or
``` ts
await database.batch(
    "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB); CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);");
```

### Insert or Update

``` ts
await database.update("INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (:integer_value, :real_value, :text_value, :blob_value)", 
new Map([
    [":integer_value", parseInt(target.integer_value.value)], 
    [":real_value", parseFloat(target.real_value.value)], 
    [":text_value", target.text_value.value],
    [":blob_value", target.blob_value.value]
]));
```

### Select

``` ts
let result = await database.select("SELECT * FROM test", new Map());
```

### Close database

``` ts
await database.close();
```

## License

[MIT](LICENSE)