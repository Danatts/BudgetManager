use std::env;

use budget_manager::{db, record::Account};
use rusqlite::Result;

fn main() -> Result<()> {
    let data = Account::build();

    let connection = db::connect()?;

    db::create_accounts_table(&connection).expect("could not create table");

    db::insert_account(&connection, &data).expect("could no insert account");

    let mut stmt = connection.prepare("SELECT * FROM accounts")?;

    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            account_id: row.get(0)?,
            value: row.get(1)?,
            description: row.get(2)?,
            entity: row.get(3)?,
            category: row.get(4)?,
        })
    })?;

    for account in account_iter {
        println!("Found record {:?}", account.unwrap());
    }

    println!("{:?}", env::args());

    Ok(())
}
