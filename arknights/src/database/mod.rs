use rusqlite::{Connection, params, Error};

fn create_database(database_file:String) -> Result<Connection>{
    let conn = Connection::open(database_file)?;
    create_table_items(&conn);
    conn
}

fn create_table_items(conn: &Connection){
    let _ = conn.execute("DROP TABLE Items", params![]);
    conn.execute(
        "CREATE TABLE Items(
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        number INTEGER NOT NULL,
    )", params![])?;
    
}