use accounting::Accounting;
use chrono::{DateTime, Local};
use core::fmt;
use rusqlite::{params, Connection, Result};

pub struct History {
    pub date: DateTime<Local>,
    pub budget_name: String,
    pub action_name: String,
    pub value: f64,
    pub description: Option<String>,
}

impl History {
    pub fn get_all_history(conn: &Connection, limit: &Option<u32>) -> Result<Vec<History>> {
        let limit = limit.unwrap_or(10);
        let query = "
            SELECT r.created_at, b.name, r.action, r.amount, r.description 
            FROM records r
            JOIN budgets b
            ON r.budget_id = b.budget_id
            ORDER BY r.created_at DESC
            LIMIT ?1
            ";
        let mut stmt = conn.prepare(query)?;
        let records_iter = stmt.query_map(params![limit], |row| {
            Ok(History {
                date: row.get(0)?,
                budget_name: row.get(1)?,
                action_name: row.get(2)?,
                value: row.get(3)?,
                description: row.get(4)?,
            })
        })?;
        records_iter.collect()
    }

    pub fn get_history_by_budget_id(
        conn: &Connection,
        budget_id: u32,
        limit: &Option<u32>,
    ) -> Result<Vec<History>> {
        let limit = limit.unwrap_or(10);
        let query = "
            SELECT r.created_at, b.name, r.action, r.amount, r.description 
            FROM records r
            JOIN budgets b
            ON r.budget_id = b.budget_id
            WHERE b.budget_id = ?1
            ORDER BY r.created_at DESC
            LIMIT ?2
            ";
        let mut stmt = conn.prepare(query)?;
        let records_iter = stmt.query_map(params![budget_id, limit], |row| {
            Ok(History {
                date: row.get(0)?,
                budget_name: row.get(1)?,
                action_name: row.get(2)?,
                value: row.get(3)?,
                description: row.get(4)?,
            })
        })?;
        records_iter.collect()
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ac = Accounting::new_from_seperator("$", 2, ".", ",");
        let date = self.date.format("%d-%m-%Y");
        let desc = match &self.description {
            Some(desc) => desc.to_owned(),
            None => String::new(),
        };
        write!(
            f,
            "{:<15}{:<20}{:<20}{:<25}{:<25}",
            date,
            self.budget_name,
            self.action_name,
            ac.format_money(self.value),
            desc
        )
    }
}

pub fn list_history(history: &Vec<History>) {
    println!(
        "\n{:<15}{:<20}{:<20}{:<25}{:<25}\n{:-^110}",
        "DATE", "BUDGET", "ACTION", "VALUE", "DESCRIPTION", ""
    );
    for record in history {
        println!("{record}");
    }
}

#[cfg(test)]
mod tests {
    use crate::{budget::Budget, record::Record};

    use super::*;

    fn setup_test_db() -> Result<Connection> {
        let mut conn = Connection::open_in_memory().unwrap();
        let tx = conn.transaction()?;
        let query = "
            CREATE TABLE IF NOT EXISTS budgets (
                budget_id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                initial_funds REAL NOT NULL,
                current_funds REAL NOT NULL
            );";
        tx.execute(query, ())?;
        let query = "
            CREATE TABLE IF NOT EXISTS records (
                record_id INTEGER PRIMARY KEY,
                budget_id INTEGER NOT NULL,
                action TEXT NOT NULL,
                amount REAL NOT NULL,
                old_value REAL NOT NULL,
                new_value REAL NOT NULL,
                description TEXT,
                created_at TEXT,
                FOREIGN KEY (budget_id) REFERENCES budgets(budget_id)
            );";
        tx.execute(query, ())?;
        tx.commit()?;
        Ok(conn)
    }

    #[test]
    fn get_all_history_ok() {
        let mut conn = setup_test_db().unwrap();
        let budget = Budget::new("budget_test", &500.0);
        let record = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        let tx = conn.transaction().unwrap();
        budget.insert_budget(&tx).unwrap();
        record.insert_record(&tx).unwrap();
        tx.commit().unwrap();
        let list = History::get_all_history(&conn, &None).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].value, 100.0);
        assert_eq!(list[0].budget_name, "Budget_test");
        assert_eq!(list[0].action_name, "action_test");
    }

    #[test]
    fn get_history_by_budget_id_ok() {
        let mut conn = setup_test_db().unwrap();
        let budget_01 = Budget::new("budget_test_01", &400.0);
        let budget_02 = Budget::new("budget_test_02", &500.0);
        let record_01 = Record::new(1, "action_test_01", 100.0, 400.0, 500.0, &None);
        let record_02 = Record::new(2, "action_test_02", 200.0, 500.0, 700.0, &None);
        let tx = conn.transaction().unwrap();
        budget_01.insert_budget(&tx).unwrap();
        budget_02.insert_budget(&tx).unwrap();
        record_01.insert_record(&tx).unwrap();
        record_02.insert_record(&tx).unwrap();
        tx.commit().unwrap();
        let list = History::get_history_by_budget_id(&conn, 2, &None).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].value, 200.0);
        assert_eq!(list[0].budget_name, "Budget_test_02");
        assert_eq!(list[0].action_name, "action_test_02");
    }
}
