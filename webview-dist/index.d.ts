export default class Rusqlite {
    name: string;
    constructor(name: string);
    static openInMemory(name: string): Promise<Rusqlite>;
    static openInPath(path: string): Promise<Rusqlite>;
    migration(migrations: Migration[]): Promise<void>;
    update(sql: string, parameters: Map<string, any>): Promise<void>;
    select(sql: string, parameters: Map<string, any>): Promise<any[]>;
    batch(batchSql: string): Promise<void>;
    close(): Promise<void>;
}
export interface Migration {
    name: string;
    sql: string;
}
