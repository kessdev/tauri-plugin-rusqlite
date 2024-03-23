import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import Rusqlite from 'tauri-plugin-rusqlite-api'

function App() {
  const [id, setId] = useState(-1);
  const [list, setList] = useState<any>([]);
  const [loadDemo, setLoadDemo] = useState(true);
  const [openDatabase, setOpenDatabase] = useState(false);

  let rusqlite: Rusqlite;
  (async() => {
    try {
      rusqlite = await Rusqlite.openInMemory("test.db");
      setLoadDemo(false);
    } catch (err) {
        console.log(err);
    }
  })();

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <div className="row">

        <div className="column">
          <p>Open Database</p>
          <form 
            onSubmit={async (e) => {
              e.preventDefault();
              let scripts = [
                {name: "create_table", sql: "CREATE TABLE test (id INTEGER PRIMARY KEY, integer_value INTEGER, real_value REAL, text_value TEXT, blob_value BLOB)"}
              ];
              await rusqlite.migration(scripts);
              setOpenDatabase(true);
            }}
          >
            <button type="submit" disabled={loadDemo} >Open In Memory</button>
          </form>
        </div>

        <div className="column">
          <p>Add data</p>
          <form 
            onSubmit={async (e) => {
              e.preventDefault();
              const target = e.target as HTMLFormElement;
              await rusqlite.update("INSERT INTO test (integer_value, real_value, text_value, blob_value) VALUES (:integer_value, :real_value, :text_value, :blob_value)", 
                new Map([[":integer_value", parseInt(target.integer_value.value)], 
                  [":real_value", parseFloat(target.real_value.value)], 
                  [":text_value", target.text_value.value],
                  [":blob_value", Array.from(target.blob_value.value, (char: string) => char.charCodeAt(0))]
                ])
              );
              const result = await rusqlite.select("SELECT last_insert_rowid() as id", new Map());
              setId(result[0].id);
              target.integer_value.value = "";
              target.real_value.value = "";
              target.text_value.value = "";
              target.blob_value.value = "";
            }}
          >
            <input type="text" placeholder="Integer" name="integer_value" disabled={!openDatabase} /> <br/>
            <input type="text" placeholder="Real" name="real_value" disabled={!openDatabase} /> <br/>
            <input type="text" placeholder="Text" name="text_value" disabled={!openDatabase} /> <br/>
            <input type="text" placeholder="Blob" name="blob_value" disabled={!openDatabase} /> <br/>
            <button type="submit" disabled={!openDatabase}>Insert</button>
            <span>{id > 0 ? 'Last id: ' + id : ''}</span>
          </form>
        </div>

        <div className="column">
          <p>Show data</p>
          <form 
            onSubmit={async (e) => {
              e.preventDefault();
              let result = await rusqlite.select("SELECT * FROM test", new Map());
              setList(result);
            }}
          >
            <button type="submit" disabled={!openDatabase}>Select</button>
          </form>
        </div>

        <div className="column">
          <p>Close Database</p>
          <form 
            onSubmit={async (e) => {
              e.preventDefault();
              await rusqlite.close();
              setOpenDatabase(false);
              setList([]);
              setId(-1);
            }}
          >
            <button type="submit" disabled={!openDatabase}>Close</button>
          </form>
        </div>

      </div>

      <div className="row">
        <table>
          <thead>
            <tr>
              <th>id</th>
              <th>integer_value</th>
              <th>real_value</th>
              <th>text_value</th>
              <th>blob_value</th>
            </tr>
          </thead>
          <tbody>
            {list.map((row: any) => (
              <tr key={row.id}>
                <td>{row.id}</td>
                <td>{row.integer_value}</td>
                <td>{row.real_value}</td>
                <td>{row.text_value}</td>
                <td>{String.fromCharCode(...row.blob_value)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default App;
