use home;
use rusqlite::Connection;
use std::process;

pub fn open_default_db() -> Connection {
    let path = home::home_dir().map(|mut home| {
        home.push("budget.test.db3");
        home
    });

    let path = match path {
        Some(path) => path,
        None => {
            eprintln!("Unable to get home directory.");
            process::exit(1);
        }
    };

    match Connection::open(path) {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Database connection failed: {}.", error);
            process::exit(1);
        }
    }
}
