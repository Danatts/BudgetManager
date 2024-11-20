use crate::history::History;
use chrono::{DateTime, Local};
use rusqlite::Connection;

pub struct Record {
    pub record_id: Option<u32>,
    pub budget_id: u32,
    pub action_id: u32,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
}

impl Record {
    pub fn new(budget_id: u32, action_id: u32, desc: &Option<String>) -> Self {
        Self {
            record_id: None,
            budget_id,
            action_id,
            description: desc.to_owned(),
            created_at: Local::now(),
        }
    }
}

pub fn create_record_table(db: &Connection) -> Result<usize, rusqlite::Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS records (
            record_id INTEGER PRIMARY KEY,
            budget_id INTEGER NOT NULL,
            action_id INTEGER NOT NULL,
            description TEXT,
            created_at TEXT,
            FOREIGN KEY (budget_id) REFERENCES budgets(budget_id)
            FOREIGN KEY (action_id) REFERENCES actions(action_id)
        );";
    db.execute(query, ())
}

pub fn insert_record(db: &Connection, record: &Record) -> Result<usize, rusqlite::Error> {
    let query = "
        INSERT INTO records (budget_id, action_id, description, created_at)
        VALUES (?1, ?2, ?3, ?4);";
    db.execute(
        query,
        (
            &record.budget_id,
            &record.action_id,
            &record.description,
            &record.created_at,
        ),
    )
}

pub fn get_records_by_budget(
    db: &Connection,
    budget_id: &u32,
) -> Result<Vec<History>, rusqlite::Error> {
    let query = "
        SELECT r.created_at, b.name, a.name, a.amount, r.description
        FROM records r
        JOIN actions a ON r.action_id = a.action_id
        JOIN budgets b ON r.budget_id = b.budget_id
        WHERE r.budget_id = ?1
        ORDER BY r.created_at DESC
        LIMIT 20;";

    let mut stmt = db.prepare(query)?;

    let record_iter = stmt.query_map([budget_id], |row| {
        Ok(History {
            date: row.get(0)?,
            budget_name: row.get(1)?,
            action_name: row.get(2)?,
            value: row.get(3)?,
            description: row.get(4)?,
        })
    })?;

    let mut history = Vec::new();

    for record in record_iter {
        history.push(record?);
    }

    Ok(history)
}

pub fn get_all_records(db: &Connection) -> Result<Vec<History>, rusqlite::Error> {
    let query = "
        SELECT r.created_at, b.name, a.name, a.amount, r.description
        FROM records r
        JOIN actions a ON r.action_id = a.action_id
        JOIN budgets b ON r.budget_id = b.budget_id
        ORDER BY r.created_at DESC
        LIMIT 20;";

    let mut stmt = db.prepare(query)?;

    let record_iter = stmt.query_map([], |row| {
        Ok(History {
            date: row.get(0)?,
            budget_name: row.get(1)?,
            action_name: row.get(2)?,
            value: row.get(3)?,
            description: row.get(4)?,
        })
    })?;

    let mut history = Vec::new();

    for record in record_iter {
        history.push(record?);
    }

    Ok(history)
}
