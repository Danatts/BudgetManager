use rusqlite::Connection;
use std::{path::PathBuf, process};

pub fn open_db(path: Option<PathBuf>) -> Connection {
    let path = match path {
        Some(path) => path,
        None => {
            if let Some(default_path) = get_default_db() {
                default_path
            } else {
                eprintln!("Could not open database");
                process::exit(1);
            }
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

fn get_default_db() -> Option<PathBuf> {
    home::home_dir().map(|mut home| {
        home.push("budget.db3");
        home
    })
}
