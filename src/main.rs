use budget_manager::{account::Account, cli::Args, db};
use clap::Parser;
use rusqlite::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let data = Account::build(&args);

    let connection = db::connect()?;

    db::create_accounts_table(&connection).expect("could not create table");

    db::insert_account(&connection, &data).expect("could no insert account");

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
