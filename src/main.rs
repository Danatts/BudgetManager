use crate::account::Account;
use crate::cli::{Action::*, CliArgs};
use clap::Parser;
use rusqlite::Connection;

pub mod account;
pub mod cli;

fn main() {
    let CliArgs { action } = CliArgs::parse();

    let connection = Connection::open_in_memory().expect("could not connect to database");

    account::create_accounts_table(&connection).expect("could not create account's table");

    match action {
        List => account::list_accounts(&connection).expect("could not list accounts"),
        Add {
            amount,
            entity,
            category,
        } => account::add_account(&connection, Account::new(amount, entity, category))
            .expect("could not add new account"),
        Update => unimplemented!(),
        Remove => unimplemented!(),
    };

    account::list_accounts(&connection).expect("could not list accounts");
}
