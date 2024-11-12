use crate::utils;
use accounting::Accounting;
use rusqlite::Connection;
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
            name: utils::capitalize(name),
            initial_funds: *funds,
            current_funds: *funds,
        }
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
        self.name = utils::capitalize(new_name);
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

pub fn create_budget_table(db: &Connection) -> Result<usize, rusqlite::Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS budgets (
            budget_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            initial_funds REAL NOT NULL,
            current_funds REAL NOT NULL
        );";
    db.execute(query, ())
}

pub fn insert_budget(db: &Connection, budget: &Budget) -> Result<usize, rusqlite::Error> {
    let query = "
        INSERT INTO budgets (name, initial_funds, current_funds)
        VALUES (?1, ?2, ?3);";
    db.execute(
        query,
        (&budget.name, &budget.initial_funds, &budget.current_funds),
    )
}

pub fn select_budget_by_id(db: &Connection, id: &u32) -> Result<Vec<Budget>, rusqlite::Error> {
    let query = "
        SELECT *
        FROM budgets
        WHERE budget_id = ?1;";
    let budget = db.query_row(query, [id], |row| {
        Ok(Budget {
            budget_id: row.get(0)?,
            name: row.get(1)?,
            initial_funds: row.get(2)?,
            current_funds: row.get(3)?,
        })
    })?;
    let budgets = vec![budget];
    Ok(budgets)
}

pub fn select_all_budgets(db: &Connection) -> Result<Vec<Budget>, rusqlite::Error> {
    let query = "
        SELECT *
        FROM budgets;";
    let mut stmt = db.prepare(query)?;
    let budget_iter = stmt.query_map([], |row| {
        Ok(Budget {
            budget_id: row.get(0)?,
            name: row.get(1)?,
            initial_funds: row.get(2)?,
            current_funds: row.get(3)?,
        })
    })?;
    let mut budgets = Vec::new();
    for budget in budget_iter {
        budgets.push(budget?);
    }
    Ok(budgets)
}

pub fn update_budget(db: &Connection, budget: &Budget) -> Result<usize, rusqlite::Error> {
    let query = "
        UPDATE budgets
        SET name = ?1,
            initial_funds = ?2,
            current_funds = ?3
        WHERE budget_id = ?4";
    db.execute(
        query,
        (
            &budget.name,
            &budget.initial_funds,
            &budget.current_funds,
            &budget.budget_id,
        ),
    )
}

pub fn delete_budget_by_id(db: &Connection, id: &u32) -> Result<usize, rusqlite::Error> {
    let query = "
        DELETE FROM budgets
        WHERE budget_id = ?1;";
    db.execute(query, [id])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.reduce_funds(&3000.0);
        assert_eq!(budget.current_funds, 2000.0);
    }

    #[test]
    fn reduce_funds_ko() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.reduce_funds(&3000.0);
        assert_ne!(budget.current_funds, 3000.0);
    }

    #[test]
    fn increase_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.increase_funds(&3000.0);
        assert_eq!(budget.current_funds, 8000.0);
    }

    #[test]
    fn increase_funds_ko() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.increase_funds(&3000.0);
        assert_ne!(budget.current_funds, 7000.0);
    }

    #[test]
    fn reset_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.increase_funds(&3000.0);
        budget.reset_funds();
        assert_eq!(budget.current_funds, budget.initial_funds);
    }

    #[test]
    fn reset_funds_ko() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.increase_funds(&3000.0);
        budget.reset_funds();
        assert_ne!(budget.current_funds, 2000.0);
    }

    #[test]
    fn set_funds_ok() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.set_current_funds(&3000.0);
        assert_eq!(budget.current_funds, 3000.0);
    }

    #[test]
    fn set_funds_ko() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.set_current_funds(&3000.0);
        assert_ne!(budget.current_funds, 5000.0);
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

    #[test]
    fn rename_ko() {
        let mut budget = Budget::new("Test", &5000.0);
        budget.rename("New name");
        assert_ne!(budget.name, String::from("Test".to_string()));
    }
}
