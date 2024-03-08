mod budget;
mod cli;
mod database;
mod transaction;
mod utils;

use cli::cli;
use database::open_default_db;

fn main() {
    let db = open_default_db();
    cli(&db);
}
