use rusqlite::{Connection, Error};

#[derive(Debug)]
pub struct Transaction {
    pub transaction_id: Option<u32>,
    pub budget_id: u32,
    pub action: String,
    pub amount: f64,
    pub desc: Option<String>,
}

impl Transaction {
    pub fn new(budget_id: &u32, action: &str, amount: &f64, desc: &Option<String>) -> Transaction {
        Transaction {
            transaction_id: None,
            budget_id: budget_id.to_owned(),
            action: action.to_string(),
            amount: amount.to_owned(),
            desc: desc.to_owned(),
        }
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
            FOREIGN KEY (budget_id) REFERENCES budgets(budget_id)
        );";
    db.execute(query, ())
}

pub fn insert_transaction(db: &Connection, transaction: &Transaction) -> Result<usize, Error> {
    let query = "
        INSERT INTO transactions (budget_id, action, amount, description)
        VALUES (?1, ?2, ?3, ?4);";
    db.execute(
        query,
        (
            &transaction.budget_id,
            &transaction.action,
            &transaction.amount,
            &transaction.desc,
        ),
    )
}

pub fn get_transactions_by_budget(
    db: &Connection,
    budget_id: &u32,
) -> Result<Vec<Transaction>, Error> {
    let query = "
        SELECT *
        FROM transactions 
        WHERE budget_id = ?1;";
    let mut stmt = db.prepare(query)?;
    let transaction_iter = stmt.query_map([budget_id], |row| {
        Ok(Transaction {
            transaction_id: row.get(0)?,
            budget_id: row.get(1)?,
            action: row.get(2)?,
            amount: row.get(3)?,
            desc: row.get(4)?,
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
        SELECT *
        FROM transactions;";
    let mut stmt = db.prepare(query)?;
    let transaction_iter = stmt.query_map([], |row| {
        Ok(Transaction {
            transaction_id: row.get(0)?,
            budget_id: row.get(1)?,
            action: row.get(2)?,
            amount: row.get(3)?,
            desc: row.get(4)?,
        })
    })?;
    let mut transaction_list = Vec::new();
    for transaction in transaction_iter {
        transaction_list.push(transaction?);
    }
    Ok(transaction_list)
}
