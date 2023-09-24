use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub struct Account {
    pub account_id: u32,
    pub value: f64,
    pub entity: String,
    pub category: String,
}

impl Account {
    pub fn new(value: f64, entity: String, category: String) -> Account {
        Account {
            account_id: 1,
            value,
            entity,
            category,
        }
    }
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

pub fn add_account(connection: &Connection, account: Account) -> Result<(), Error> {
    let query = "INSERT INTO accounts VALUES(?1, ?2, ?3, ?4)";
    let _ = connection.execute(
        query,
        (
            &account.account_id,
            &account.value,
            &account.entity,
            &account.category,
        ),
    );
    println!("account added");
    Ok(())
}

pub fn list_accounts(connection: &Connection) -> Result<(), Error> {
    let mut stmt = connection.prepare("SELECT * FROM accounts")?;

    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            account_id: row.get(0)?,
            value: row.get(1)?,
            entity: row.get(2)?,
            category: row.get(3)?,
        })
    })?;

    for account in account_iter {
        println!("{:?}", account.unwrap());
    }

    Ok(())
}

pub fn update_account() {
    unimplemented!()
}

pub fn remove_account() {
    unimplemented!()
}
