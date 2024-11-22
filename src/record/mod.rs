use chrono::{DateTime, Local};
use rusqlite::{params, Connection, Result};

pub struct Record {
    pub record_id: Option<u32>,
    pub budget_id: u32,
    pub action: String,
    pub amount: f64,
    pub old_value: f64,
    pub new_value: f64,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
}

impl Record {
    pub fn new(
        budget_id: u32,
        action: &str,
        amount: f64,
        old_value: f64,
        new_value: f64,
        desc: &Option<String>,
    ) -> Self {
        Self {
            record_id: None,
            budget_id,
            action: action.to_string(),
            amount,
            old_value,
            new_value,
            description: desc.to_owned(),
            created_at: Local::now(),
        }
    }

    pub fn insert_record(&self, conn: &Connection) -> Result<u32> {
        let query = "
            INSERT INTO records (budget_id, action, amount, old_value,
            new_value, description, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ";
        conn.execute(
            query,
            params![
                &self.budget_id,
                &self.action,
                &self.amount,
                &self.old_value,
                &self.new_value,
                &self.description,
                &self.created_at,
            ],
        )?;
        Ok(conn.last_insert_rowid() as u32)
    }

    pub fn get_record_by_id(conn: &Connection, record_id: u32) -> Result<Record> {
        let query = "
            SELECT record_id, budget_id, action, amount, old_value, new_value, description, created_at
            FROM records
            WHERE record_id = ?1
            ";
        conn.query_row(query, params![record_id], |row| {
            Ok(Record {
                record_id: row.get(0)?,
                budget_id: row.get(1)?,
                action: row.get(2)?,
                amount: row.get(3)?,
                old_value: row.get(4)?,
                new_value: row.get(5)?,
                description: row.get(6)?,
                created_at: row.get(7)?,
            })
        })
    }

    pub fn get_all_records(conn: &Connection) -> Result<Vec<Record>> {
        let query = "
            SELECT record_id, budget_id, action, amount, old_value, new_value, description, created_at
            FROM records
            ";
        let mut stmt = conn.prepare(query)?;
        let records_iter = stmt.query_map(params![], |row| {
            Ok(Record {
                record_id: row.get(0)?,
                budget_id: row.get(1)?,
                action: row.get(2)?,
                amount: row.get(3)?,
                old_value: row.get(4)?,
                new_value: row.get(5)?,
                description: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        records_iter.collect()
    }

    pub fn get_records_by_budget_id(conn: &Connection, budget_id: u32) -> Result<Vec<Record>> {
        let query = "
            SELECT record_id, budget_id, action, amount, old_value, new_value, description, created_at
            FROM records
            WHERE budget_id = ?1
            ";
        let mut stmt = conn.prepare(query)?;
        let records_iter = stmt.query_map(params![budget_id], |row| {
            Ok(Record {
                record_id: row.get(0)?,
                budget_id: row.get(1)?,
                action: row.get(2)?,
                amount: row.get(3)?,
                old_value: row.get(4)?,
                new_value: row.get(5)?,
                description: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        records_iter.collect()
    }

    pub fn delete_record_by_id(conn: &Connection, record_id: u32) -> Result<usize> {
        let query = "
            DELETE
            FROM records
            WHERE budget_id = ?1
            ";
        conn.execute(query, params![record_id])
    }
}

pub fn create_record_table(conn: &Connection) -> Result<()> {
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
        )";
    conn.execute(query, ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        let query = "
            CREATE TABLE IF NOT EXISTS records (
                record_id INTEGER PRIMARY KEY,
                budget_id INTEGER NOT NULL,
                action TEXT NOT NULL,
                amount REAL NOT NULL,
                old_value REAL NOT NULL,
                new_value REAL NOT NULL,
                description TEXT,
                created_at TEXT
            );";
        conn.execute(query, ())?;
        Ok(conn)
    }

    #[test]
    fn insert_record_ok() {
        let conn = setup_test_db().unwrap();
        let record_01 = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        let id_01 = record_01.insert_record(&conn).unwrap();
        let record_02 = Record::new(2, "action_test", 200.0, 600.0, 800.0, &None);
        let id_02 = record_02.insert_record(&conn).unwrap();
        assert_eq!(id_01, 1);
        assert_eq!(id_02, 2);
    }

    #[test]
    fn get_record_by_id_ok() {
        let conn = setup_test_db().unwrap();
        let record = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        record.insert_record(&conn).unwrap();
        let record = Record::get_record_by_id(&conn, 1).unwrap();
        assert_eq!(record.record_id, Some(1));
        assert_eq!(record.budget_id, 1);
        assert_eq!(record.action, "action_test");
        assert_eq!(record.amount, 100.0);
        assert_eq!(record.old_value, 500.0);
        assert_eq!(record.new_value, 600.0);
        assert_eq!(record.description, None);
    }

    #[test]
    fn get_record_by_id_ko() {
        let conn = setup_test_db().unwrap();
        let record = Record::get_record_by_id(&conn, 1);
        assert!(record.is_err_and(|x| x == rusqlite::Error::QueryReturnedNoRows));
    }

    #[test]
    fn get_all_records_ok() {
        let conn = setup_test_db().unwrap();
        let record_01 = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        let record_02 = Record::new(2, "action_test", 200.0, 600.0, 800.0, &None);
        record_01.insert_record(&conn).unwrap();
        record_02.insert_record(&conn).unwrap();
        let records = Record::get_all_records(&conn).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].record_id, Some(1));
        assert_eq!(records[1].record_id, Some(2));
    }

    #[test]
    fn get_all_records_empty() {
        let conn = setup_test_db().unwrap();
        let records = Record::get_all_records(&conn).unwrap();
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn get_records_by_budget_id_ok() {
        let conn = setup_test_db().unwrap();
        let record_01 = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        let record_02 = Record::new(1, "action_test", 200.0, 600.0, 800.0, &None);
        let record_03 = Record::new(2, "action_test", 300.0, 700.0, 1000.0, &None);
        record_01.insert_record(&conn).unwrap();
        record_02.insert_record(&conn).unwrap();
        record_03.insert_record(&conn).unwrap();
        let records = Record::get_records_by_budget_id(&conn, 1).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].record_id, Some(1));
        assert_eq!(records[1].record_id, Some(2));
    }

    #[test]
    fn get_records_by_budget_id_empty() {
        let conn = setup_test_db().unwrap();
        let record = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        record.insert_record(&conn).unwrap();
        let records = Record::get_records_by_budget_id(&conn, 2).unwrap();
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn delete_record_by_id_ok() {
        let conn = setup_test_db().unwrap();
        let record = Record::new(1, "action_test", 100.0, 500.0, 600.0, &None);
        record.insert_record(&conn).unwrap();
        let res = Record::delete_record_by_id(&conn, 1).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn delete_record_by_id_empty() {
        let conn = setup_test_db().unwrap();
        let res = Record::delete_record_by_id(&conn, 1).unwrap();
        assert_eq!(res, 0);
    }
}
