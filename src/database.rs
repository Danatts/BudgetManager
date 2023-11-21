use std::process;
use rusqlite::Connection;

pub fn open_db(path: &str) -> Connection {
    match Connection::open(path) {
        Ok(db) => db,
        Err(error) => {
            eprintln!("database connection failed: {}", error);
            process::exit(1);
        }
    }
}
