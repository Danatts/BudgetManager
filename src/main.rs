mod budget;
mod cli;
mod database;
mod services;
mod transaction;
mod utils;

use clap::Parser;
use cli::{run, Cli};
use database::open_db;

fn main() {
    let Cli { command, database } = Cli::parse();
    let db = open_db(database);
    run(db, command);
}
