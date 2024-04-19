import { invoke } from '@tauri-apps/api/tauri'

export default class Rusqlite {
  name: string;

  constructor(name: string) {
    this.name = name;
  }
  
  static async openInMemory(name: string): Promise<Rusqlite> {
    return await invoke('plugin:rusqlite|open_in_memory', {name: name}).then(() => new Rusqlite(name));
  }

  static async openInPath(path: string): Promise<Rusqlite> {
    return await invoke('plugin:rusqlite|open_in_path', {path: path}).then(() => new Rusqlite(path));
  }

  async migration(migrations: Migration[]): Promise<void> {
    return await invoke('plugin:rusqlite|migration', {name: this.name, migrations});
  }
  
  async update(sql: string, parameters: Map<string, any>): Promise<void> {
    return await invoke('plugin:rusqlite|update', {name: this.name, sql, parameters});
  }

  async select(sql: string, parameters:Map<string, any>): Promise<any[]> {
    return await invoke('plugin:rusqlite|select', {name: this.name, sql, parameters});
  }

  async batch(sql: string): Promise<void> {
    return await invoke('plugin:rusqlite|batch', {name: this.name, sql});
  }

  async close(): Promise<void> {
    return await invoke('plugin:rusqlite|close', {name: this.name});
  }
}

export interface Migration {
  name: string;
  sql: string;
}
