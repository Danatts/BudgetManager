use budget_manager::cli;
use budget_manager::cli::Cli;
use budget_manager::database;
use clap::Parser;

fn main() {
    let Cli { command, database } = Cli::parse();
    let mut conn = database::open_db(database);
    cli::run(&mut conn, command);
}
