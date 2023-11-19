use rusqlite::Connection;

pub fn open_db(path: &str) -> Connection {
    let db = match Connection::open(path) {
        Ok(db) => db,
        Err(error) => panic!("database connection failed: {}", error),
    };
    db
}
