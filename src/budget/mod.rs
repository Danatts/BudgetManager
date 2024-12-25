use crate::utils;
use accounting::Accounting;
use rusqlite::{params, Connection, Result, Transaction};
use std::fmt;

pub struct Budget {
    pub budget_id: Option<u32>,
    pub name: String,
    pub initial_funds: f64,
    pub current_funds: f64,
}

impl Budget {
    pub fn new(name: &str, funds: &f64) -> Self {
        Self {
            budget_id: None,
            name: utils::upper(name),
            initial_funds: *funds,
            current_funds: *funds,
        }
    }

    pub fn insert_budget(&self, conn: &Connection) -> Result<usize> {
        let query = "
            INSERT INTO budgets (name, initial_funds, current_funds)
            VALUES (?1, ?2, ?3)
            ";
        conn.execute(
            query,
            params![&self.name, &self.initial_funds, &self.current_funds],
        )?;
        Ok(conn.last_insert_rowid() as usize)
    }

    pub fn get_budget_by_id(conn: &Connection, budget_id: &u32) -> Result<Budget> {
        let query = "
            SELECT *
            FROM budgets
            WHERE budget_id = ?1
            ";
        conn.query_row(query, params![budget_id], |row| {
            Ok(Budget {
                budget_id: row.get(0)?,
                name: row.get(1)?,
                initial_funds: row.get(2)?,
                current_funds: row.get(3)?,
            })
        })
    }

    pub fn get_all_budgets(conn: &Connection) -> Result<Vec<Budget>> {
        let query = "
            SELECT *
            FROM budgets
            ";
        let mut stmt = conn.prepare(query)?;
        let budgets_iter = stmt.query_map(params![], |row| {
            Ok(Budget {
                budget_id: row.get(0)?,
                name: row.get(1)?,
                initial_funds: row.get(2)?,
                current_funds: row.get(3)?,
            })
        })?;
        budgets_iter.collect()
    }

    pub fn update_budget(&self, conn: &Connection) -> Result<usize> {
        let query = "
            UPDATE budgets
            SET name = ?1,
                initial_funds = ?2,
                current_funds = ?3
            WHERE budget_id = ?4
            ";
        conn.execute(
            query,
            params![
                &self.name,
                &self.initial_funds,
                &self.current_funds,
                &self.budget_id,
            ],
        )
    }

    pub fn update_budget_tx(&self, tx: &Transaction) -> Result<usize> {
        let query = "
            UPDATE budgets
            SET name = ?1,
                initial_funds = ?2,
                current_funds = ?3
            WHERE budget_id = ?4
            ";
        tx.execute(
            query,
            params![
                &self.name,
                &self.initial_funds,
                &self.current_funds,
                &self.budget_id,
            ],
        )
    }

    pub fn delete_budget_by_id(conn: &Connection, budget_id: &u32) -> Result<usize> {
        let query = "
            DELETE 
            FROM budgets
            WHERE budget_id = ?1
            ";
        conn.execute(query, params![budget_id])
    }

    pub fn increase_funds(&mut self, amount: &f64) {
        self.current_funds += amount;
    }

    pub fn reduce_funds(&mut self, amount: &f64) {
        self.current_funds -= amount;
    }

    pub fn reset_funds(&mut self) {
        self.current_funds = self.initial_funds;
    }

    pub fn set_current_funds(&mut self, amount: &f64) {
        self.current_funds = *amount;
    }

    pub fn set_initial_funds(&mut self, amount: &f64) {
        self.initial_funds = *amount;
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = utils::upper(new_name);
    }
}

impl fmt::Display for Budget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ac = Accounting::new_from_seperator("$", 2, ".", ",");
        write!(
            f,
            "{:<5}{:<20}{:>25}{:>25}",
            self.budget_id.unwrap(),
            self.name,
            ac.format_money(self.current_funds),
            ac.format_money(self.initial_funds)
        )
    }
}

pub fn list_budgets(budgets: &Vec<Budget>) {
    println!(
        "\n{:<5}{:<20}{:>25}{:>25}\n{:-^80}",
        "ID", "BUDGET", "CURRENT FUNDS", "INITIAL FUNDS", ""
    );

    for budget in budgets {
        println!("{budget}")
    }
}

pub fn create_budget_table(conn: &Connection) -> Result<()> {
    let query = "
        CREATE TABLE IF NOT EXISTS budgets (
            budget_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            initial_funds REAL NOT NULL,
            current_funds REAL NOT NULL
        );";
    conn.execute(query, ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        create_budget_table(&conn)?;
        Ok(conn)
    }

    #[test]
    fn insert_budget_ok() {
        let conn = setup_test_db().unwrap();
        let budget_01 = Budget::new("budget_test1", &500.0);
        let id_01 = budget_01.insert_budget(&conn).unwrap();
        let budget_02 = Budget::new("budget_test2", &600.0);
        let id_02 = budget_02.insert_budget(&conn).unwrap();
        assert_eq!(id_01, 1);
        assert_eq!(id_02, 2);
    }

    #[test]
    fn get_budget_by_id_ok() {
        let conn = setup_test_db().unwrap();
        let budget = Budget::new("budget_test", &500.0);
        budget.insert_budget(&conn).unwrap();
        let budget = Budget::get_budget_by_id(&conn, &1).unwrap();
        assert_eq!(budget.budget_id, Some(1));
        assert_eq!(budget.name, "Budget_test");
        assert_eq!(budget.current_funds, 500.0);
        assert_eq!(budget.initial_funds, 500.0);
    }

    #[test]
    fn get_budget_by_id_ko() {
        let conn = setup_test_db().unwrap();
        let record = Budget::get_budget_by_id(&conn, &1);
        assert!(record.is_err_and(|x| x == rusqlite::Error::QueryReturnedNoRows));
    }

    #[test]
    fn get_all_budgets_ok() {
        let conn = setup_test_db().unwrap();
        let budget_01 = Budget::new("budget_test1", &500.0);
        let budget_02 = Budget::new("budget_test2", &600.0);
        budget_01.insert_budget(&conn).unwrap();
        budget_02.insert_budget(&conn).unwrap();
        let budgets = Budget::get_all_budgets(&conn).unwrap();
        assert_eq!(budgets.len(), 2);
        assert_eq!(budgets[0].budget_id, Some(1));
        assert_eq!(budgets[1].budget_id, Some(2));
    }

    #[test]
    fn get_all_budgets_empty() {
        let conn = setup_test_db().unwrap();
        let budgets = Budget::get_all_budgets(&conn).unwrap();
        assert_eq!(budgets.len(), 0);
    }

    #[test]
    fn update_budget_by_id_ok() {
        let conn = setup_test_db().unwrap();
        let budget = Budget::new("budget_test", &500.0);
        budget.insert_budget(&conn).unwrap();
        let mut budget = Budget::get_budget_by_id(&conn, &1).unwrap();
        budget.reduce_funds(&200.0);
        let res = budget.update_budget(&conn).unwrap();
        let budget = Budget::get_budget_by_id(&conn, &1).unwrap();
        assert_eq!(res, 1);
        assert_eq!(budget.current_funds, 300.0);
    }

    #[test]
    fn update_budget_by_id_empty() {
        let conn = setup_test_db().unwrap();
        let budget = Budget::new("budget_test", &500.0);
        let res = budget.update_budget(&conn).unwrap();
        assert_eq!(res, 0);
    }

    #[test]
    fn delete_budget_by_id_ok() {
        let conn = setup_test_db().unwrap();
        let budget = Budget::new("budget_test", &500.0);
        budget.insert_budget(&conn).unwrap();
        let res = Budget::delete_budget_by_id(&conn, &1).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn delete_budget_by_id_empty() {
        let conn = setup_test_db().unwrap();
        let res = Budget::delete_budget_by_id(&conn, &1).unwrap();
        assert_eq!(res, 0);
    }

    #[test]
    fn reduce_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.reduce_funds(&3000.0);
        assert_eq!(budget.current_funds, 2000.0);
    }

    #[test]
    fn increase_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.increase_funds(&3000.0);
        assert_eq!(budget.current_funds, 8000.0);
    }
    #[test]
    fn reset_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.increase_funds(&3000.0);
        budget.reset_funds();
        assert_eq!(budget.current_funds, budget.initial_funds);
    }

    #[test]
    fn set_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.set_current_funds(&3000.0);
        assert_eq!(budget.current_funds, 3000.0);
    }
    #[test]
    fn set_initial_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.set_initial_funds(&3000.0);
        assert_eq!(budget.initial_funds, 3000.0);
    }

    #[test]
    fn set_initial_ko() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.set_initial_funds(&3000.0);
        assert_ne!(budget.initial_funds, 5000.0);
    }

    #[test]
    fn rename_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.rename("New name");
        assert_eq!(budget.name, "New name");
    }
}
