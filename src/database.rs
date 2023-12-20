use home;
use rusqlite::Connection;
use std::process;

// TODO:
// 1. el usuario pueda dar una ruta que quiera
// 2. implementar elección de la base de datos depende del entorno
pub fn open_default_db() -> Connection {
    let path = home::home_dir().map(|mut home| {
        home.push("budget.test.db3");
        home
    });

    let path = match path {
        Some(path) => path,
        None => {
            eprintln!("Unable to get home directory");
            process::exit(1);
        }
    };

    match Connection::open(path) {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Database connection failed: {}", error);
            process::exit(1);
        }
    }
}
