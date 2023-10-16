use std::process::Termination;

use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Account {
    pub account_id: Option<i32>,
    pub amount: f64,
    pub entity: String,
    pub category: String,
}

impl Account {
    pub fn new(amount: f64, entity: String, category: String) -> Account {
        Account {
            account_id: None,
            amount,
            entity,
            category,
        }
    }
}

impl Termination for Account {
    fn report(self) -> std::process::ExitCode {
        unimplemented!()
    }
}

pub fn create_accounts_table(connection: &Connection) -> Result<usize> {
    let query = "
        CREATE TABLE accounts (
            account_id INTEGER PRIMARY KEY,
            amount REAL,
            entity TEXT,
            category TEXT
        )";
    connection.execute(query, ())
}

pub fn add_account(connection: &Connection, account: Account) -> Result<usize> {
    let query = "INSERT INTO accounts VALUES(?1, ?2, ?3, ?4)";
    connection.execute(
        query,
        (
            &account.account_id,
            &account.amount,
            &account.entity,
            &account.category,
        ),
    )
}

pub fn list_accounts(connection: &Connection) -> Result<usize> {
    let mut stmt = connection.prepare("SELECT * FROM accounts")?;

    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            account_id: row.get(0)?,
            amount: row.get(1)?,
            entity: row.get(2)?,
            category: row.get(3)?,
        })
    })?;

    for account in account_iter {
        println!("{:?}", account.unwrap());
    }

    Ok(0)
}

pub fn update_account() {
    unimplemented!()
}

pub fn remove_account() {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
