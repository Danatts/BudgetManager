use crate::account::Account;
use rusqlite::{Connection, Error};

pub fn connect() -> Result<Connection, Error> {
    let connection = Connection::open_in_memory();
    connection
}

pub fn create_accounts_table(connection: &Connection) -> Result<usize, Error> {
    let query = "
        CREATE TABLE accounts (
            account_id INTEGER PRIMARY KEY,
            value REAL,
            entity TEXT,
            category TEXT
        )";
    connection.execute(query, ())
}

pub fn insert_account(connection: &Connection, account: &Account) -> Result<usize, Error> {
    let query = "INSERT INTO accounts VALUES(?1, ?2, ?3, ?4)";
    connection.execute(
        query,
        (
            &account.account_id,
            &account.value,
            &account.entity,
            &account.category,
        ),
    )
}
