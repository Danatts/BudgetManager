use crate::utils;
use accounting::Accounting;
use chrono::{DateTime, Local};
use rusqlite::Connection;

pub struct Record {
    pub record_id: Option<u32>,
    pub budget_id: u32,
    pub budget_name: Option<String>,
    pub action: String,
    pub amount: f64,
    pub desc: Option<String>,
    pub created_at: DateTime<Local>,
}

impl Record {
    pub fn new(budget_id: &u32, action: &str, amount: &f64, desc: &Option<String>) -> Record {
        Record {
            record_id: None,
            budget_id: *budget_id,
            budget_name: None,
            action: utils::capitalize(action),
            amount: *amount,
            desc: desc.to_owned(),
            created_at: Local::now(),
        }
    }
}

pub fn print_records(records: &Vec<Record>) {
    let ac = Accounting::new_from_seperator("$", 2, ".", ",");
    println!(
        "\n{:<15}{:<20}{:<20}{:<25}{:<25}\n{:-^110}",
        "DATE", "BUDGET", "ACTION", "VALUE", "DESCRIPTION", ""
    );
    for record in records {
        let bud_name = match &record.budget_name {
            Some(text) => text.to_owned(),
            None => String::new(),
        };
        let desc = match &record.desc {
            Some(text) => text.to_owned(),
            None => String::new(),
        };
        let date = record.created_at.format("%d-%m-%Y");
        println!(
            "{:<15}{:<20}{:<20}{:<25}{:<25}",
            date,
            bud_name,
            record.action,
            ac.format_money(record.amount),
            desc,
        )
    }
}

pub fn create_record_table(db: &Connection) -> Result<usize, rusqlite::Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS records (
            record_id INTEGER PRIMARY KEY,
            budget_id INTEGER NOT NULL,
            action TEXT NOT NULL,
            amount REAL NOT NULL,
            description TEXT,
            created_at TEXT,
            FOREIGN KEY (budget_id) REFERENCES budgets(budget_id)
        );";
    db.execute(query, ())
}
pub fn insert_record(db: &Connection, record: &Record) -> Result<usize, rusqlite::Error> {
    let query = "
        INSERT INTO records (budget_id, action, amount, description, created_at)
        VALUES (?1, ?2, ?3, ?4, ?5);";
    db.execute(
        query,
        (
            &record.budget_id,
            &record.action,
            &record.amount,
            &record.desc,
            &record.created_at,
        ),
    )
}

pub fn get_records_by_budget(
    db: &Connection,
    budget_id: &u32,
) -> Result<Vec<Record>, rusqlite::Error> {
    let query = "
        SELECT t.record_id, b.budget_id, b.name, t.action, t.amount,  t.description, t.created_at
        FROM records t
        JOIN budgets b 
        ON t.budget_id = b.budget_id
        WHERE t.budget_id = ?1
        ORDER BY t.created_at DESC
        LIMIT 30;";
    let mut stmt = db.prepare(query)?;
    let record_iter = stmt.query_map([budget_id], |row| {
        Ok(Record {
            record_id: row.get(0)?,
            budget_id: row.get(1)?,
            budget_name: row.get(2)?,
            action: row.get(3)?,
            amount: row.get(4)?,
            desc: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut records_list = Vec::new();
    for record in record_iter {
        records_list.push(record?);
    }
    Ok(records_list)
}

pub fn get_all_records(db: &Connection) -> Result<Vec<Record>, rusqlite::Error> {
    let query = "
        SELECT t.record_id, b.budget_id, b.name, t.action, t.amount,  t.description, t.created_at
        FROM records t
        JOIN budgets b 
        ON t.budget_id = b.budget_id
        ORDER BY t.created_at DESC
        LIMIT 30;";
    let mut stmt = db.prepare(query)?;
    let record_iter = stmt.query_map([], |row| {
        Ok(Record {
            record_id: row.get(0)?,
            budget_id: row.get(1)?,
            budget_name: row.get(2)?,
            action: row.get(3)?,
            amount: row.get(4)?,
            desc: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut records_list = Vec::new();
    for record in record_iter {
        records_list.push(record?);
    }
    Ok(records_list)
}
