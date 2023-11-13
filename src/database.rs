use crate::budget::Budget;
use rusqlite::Connection;

pub fn open_db(path: &str) -> Connection {
    let db = match Connection::open(path) {
        Ok(db) => db,
        Err(error) => panic!("database connection failed: {}", error),
    };
    db
}

pub fn create_budget_table(db: &Connection) {
    let query = "
        CREATE TABLE IF NOT EXISTS budgets (
            budget_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            initial_funds REAL NOT NULL,
            current_funds REAL NOT NULL
        );";
    match db.execute(query, ()) {
        Ok(_) => println!("table created successfully."),
        Err(error) => eprintln!("creation failed: {}", error),
    }
}

pub fn insert_new_budget(db: &Connection, budget: &Budget) {
    let query = "
        INSERT INTO budgets (name, initial_funds, current_funds)
        VALUES (?1, ?2, ?3);";
    match db.execute(
        query,
        (&budget.name, &budget.initial_funds, &budget.current_funds),
    ) {
        Ok(rows) => println!("{} rows were inserted.", rows),
        Err(error) => eprintln!("insertion failed: {}", error),
    }
}

pub fn get_budget_by_id(db: &Connection, id: u32) -> Result<Budget, ()> {
    let query = "
        SELECT *
        FROM budgets
        WHERE budget_id = ?1;";
    match db.query_row(query, [id], |row| {
        Ok(Budget {
            budget_id: row.get(0)?,
            name: row.get(1)?,
            initial_funds: row.get(2)?,
            current_funds: row.get(3)?,
        })
    }) {
        Ok(budget) => Ok(budget),
        Err(error) => Err(eprintln!("could not get budget: {}", error)),
    }
}

pub fn update_budget(db: &Connection, budget: &Budget) {
    let query = "
        UPDATE budgets
        SET name = ?1,
            initial_funds = ?2,
            current_funds = ?3
        WHERE budget_id = ?4";
    match db.execute(
        query,
        (
            &budget.name,
            &budget.initial_funds,
            &budget.current_funds,
            &budget.budget_id.unwrap(),
        ),
    ) {
        Ok(rows) => println!("{} rows were updated.", rows),
        Err(error) => eprintln!("update failed: {}", error),
    }
}

pub fn delete_budget_by_id(db: &Connection, id: u32) {
    let query = "
        DELETE FROM budgets
        WHERE budget_id = ?1;";
    match db.execute(query, [id]) {
        Ok(rows) => println!("{} rows were deleted.", rows),
        Err(error) => eprintln!("deletion failed: {}", error),
    }
}
