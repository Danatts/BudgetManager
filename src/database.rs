use rusqlite::{Connection, Error};

pub fn open_db(path: &str) -> Result<Connection, Error> {
    let db = Connection::open(path)?;
    Ok(db)
}

pub fn create_budget_table(db: &Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS budgets (
            budget_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            initial_funds REAL NOT NULL,
            current_funds REAL NOT NULL
        )";
    db.execute(query, ())
        .expect("could not create budget table");
}
