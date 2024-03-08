use accounting::Accounting;
use chrono::{DateTime, Local};
use std::fmt;

use crate::utils::capitalize;
use rusqlite::{Connection, Error};

pub struct Transaction {
    pub transaction_id: Option<u32>,
    pub budget_id: u32,
    pub budget_name: Option<String>,
    pub action: String,
    pub amount: f64,
    pub desc: Option<String>,
    pub created_at: DateTime<Local>,
}

impl Transaction {
    pub fn new(budget_id: &u32, action: &str, amount: &f64, desc: &Option<String>) -> Transaction {
        Transaction {
            transaction_id: None,
            budget_id: *budget_id,
            budget_name: None,
            action: capitalize(action),
            amount: *amount,
            desc: desc.to_owned(),
            created_at: Local::now(),
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ac = Accounting::new_from_seperator("$", 2, ".", ",");
        let bud_name = match &self.budget_name {
            Some(text) => text.to_owned(),
            None => String::new(),
        };
        let desc = match &self.desc {
            Some(text) => text.to_owned(),
            None => String::new(),
        };
        let date = self.created_at.format("%d-%m-%Y");
        write!(
            f,
            " {:<15}{:<20}{:<20}{:<25}{:<25}",
            date,
            bud_name,
            self.action,
            ac.format_money(self.amount),
            desc,
        )
    }
}

pub fn print_transactions(transactions: &Vec<Transaction>) {
    println!(
        "\n {:<15}{:<20}{:<20}{:<25}{:<25}\n{:-^110}",
        "DATE", "BUDGET", "ACTION", "VALUE", "DESCRIPTION", ""
    );
    for transaction in transactions {
        println!("{transaction}")
    }
}

pub fn create_transaction_table(db: &Connection) -> Result<usize, Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS transactions (
            transaction_id INTEGER PRIMARY KEY,
            budget_id INTEGER NOT NULL,
            action TEXT NOT NULL,
            amount REAL NOT NULL,
            description TEXT,
            created_at TEXT,
            FOREIGN KEY (budget_id) REFERENCES budgets(budget_id)
        );";
    db.execute(query, ())
}
pub fn insert_transaction(db: &Connection, transaction: &Transaction) -> Result<usize, Error> {
    let query = "
        INSERT INTO transactions (budget_id, action, amount, description, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5);";
    db.execute(
        query,
        (
            &transaction.budget_id,
            &transaction.action,
            &transaction.amount,
            &transaction.desc,
            &transaction.created_at,
        ),
    )
}

pub fn get_transactions_by_budget(
    db: &Connection,
    budget_id: &u32,
) -> Result<Vec<Transaction>, Error> {
    let query = "
        SELECT t.transaction_id, b.budget_id, b.name, t.action, t.amount,  t.description, t.created_at
        FROM transactions t
        JOIN budgets b 
        ON t.budget_id = b.budget_id
        WHERE t.budget_id = ?1
        ORDER BY t.created_at DESC
        LIMIT 30;";
    let mut stmt = db.prepare(query)?;
    let transaction_iter = stmt.query_map([budget_id], |row| {
        Ok(Transaction {
            transaction_id: row.get(0)?,
            budget_id: row.get(1)?,
            budget_name: row.get(2)?,
            action: row.get(3)?,
            amount: row.get(4)?,
            desc: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut transaction_list = Vec::new();
    for transaction in transaction_iter {
        transaction_list.push(transaction?);
    }
    Ok(transaction_list)
}

pub fn get_all_transactions(db: &Connection) -> Result<Vec<Transaction>, Error> {
    let query = "
        SELECT t.transaction_id, b.budget_id, b.name, t.action, t.amount,  t.description, t.created_at
        FROM transactions t
        JOIN budgets b 
        ON t.budget_id = b.budget_id
        ORDER BY t.created_at DESC
        LIMIT 30;";
    let mut stmt = db.prepare(query)?;
    let transaction_iter = stmt.query_map([], |row| {
        Ok(Transaction {
            transaction_id: row.get(0)?,
            budget_id: row.get(1)?,
            budget_name: row.get(2)?,
            action: row.get(3)?,
            amount: row.get(4)?,
            desc: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut transaction_list = Vec::new();
    for transaction in transaction_iter {
        transaction_list.push(transaction?);
    }
    Ok(transaction_list)
}
