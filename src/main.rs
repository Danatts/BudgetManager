pub mod budget;
pub mod cli;
pub mod database;
pub mod transaction;
pub mod utils;

use cli::cli;
use database::open_default_db;

fn main() {
    let db = open_default_db();
    cli(&db);
}
