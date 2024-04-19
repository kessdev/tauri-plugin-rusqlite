import Rusqlite from "tauri-plugin-rusqlite-api";

let rusqlite: Rusqlite | null = null;

export async function rusqliteOpenInMemory(): Promise<Rusqlite> {
    if (!rusqlite) {
        rusqlite = await Rusqlite.openInMemory("test.db");
    }
    return rusqlite;
}

export async function rusqliteOpenInPath(databasePath: string): Promise<Rusqlite> {
    if (!rusqlite) {
        rusqlite = await Rusqlite.openInPath(databasePath);
    }
    return rusqlite;
}

export async function rusqliteClose(): Promise<void> {
    rusqlite = null;
}