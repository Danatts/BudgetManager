use crate::budget::Budget;
use rusqlite::{Connection, Error};

pub fn open_db(path: &str) -> Result<Connection, Error> {
    let db = Connection::open(path)?;
    Ok(db)
}

pub fn create_budget_table(db: &Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS budgets (
            budget_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            initial_funds REAL NOT NULL,
            current_funds REAL NOT NULL
        );";
    db.execute(query, ())
        .expect("could not create budget table");
}

pub fn insert_new_budget(db: &Connection, budget: &Budget) {
    let query = "
        INSERT INTO budgets (name, initial_funds, current_funds)
        VALUES (?1, ?2, ?3);";
    db.execute(
        query,
        (&budget.name, &budget.initial_funds, &budget.current_funds),
    )
    .expect("could not insert new budget to database");
}

pub fn update_budget(db: &Connection, budget: &Budget) {
    let query = "
        UPDATE budgets
        SET name = ?1,
            initial_funds = ?2,
            current_funds = ?3
        WHERE
            budget_id = ?4";
    db.execute(
        query,
        (
            &budget.name,
            &budget.initial_funds,
            &budget.current_funds,
            &budget.budget_id.unwrap(),
        ),
    )
    .expect("could not update budget in database");
}
