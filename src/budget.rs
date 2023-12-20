use crate::utils::capitalize;
use accounting::Accounting;
use rusqlite::{Connection, Error};
use std::fmt;

pub struct Budget {
    pub budget_id: Option<u32>,
    pub name: String,
    pub initial_funds: f64,
    pub current_funds: f64,
}

impl Budget {
    pub fn new(name: &str, funds: f64) -> Budget {
        Budget {
            budget_id: None,
            name: capitalize(&name),
            initial_funds: funds,
            current_funds: funds,
        }
    }

    pub fn increase_funds(&mut self, amount_to_increase: &f64) {
        self.current_funds += amount_to_increase;
    }

    pub fn reduce_funds(&mut self, amount_to_reduce: &f64) {
        self.current_funds -= amount_to_reduce;
    }

    pub fn reset_funds(&mut self) {
        self.current_funds = self.initial_funds;
    }

    pub fn set_current_funds(&mut self, amount_to_set: &f64) {
        self.current_funds = amount_to_set.to_owned();
    }

    pub fn set_initial_funds(&mut self, amount_to_set: &f64) {
        self.initial_funds = amount_to_set.to_owned();
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

impl fmt::Display for Budget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ac = Accounting::new_from_seperator("$", 2, ".", ",");
        write!(
            f,
            " {:<5}{:<20}{:>25}{:>25}",
            self.budget_id.unwrap(),
            self.name,
            ac.format_money(self.current_funds),
            ac.format_money(self.initial_funds)
        )
    }
}

pub fn print_budget(budget: &Budget) {
    println!(
        "\n {:<5}{:<20}{:>25}{:>25}\n{:-^80}\n{budget}",
        "ID", "BUDGET", "CURRENT FUNDS", "INITIAL FUNDS", ""
    );
}

pub fn print_all_budgets(budgets: &Vec<Budget>) {
    println!(
        "\n {:<5}{:<20}{:>25}{:>25}\n{:-^80}",
        "ID", "BUDGET", "CURRENT FUNDS", "INITIAL FUNDS", ""
    );
    for budget in budgets {
        println!("{budget}")
    }
}

pub fn create_budget_table(db: &Connection) -> Result<usize, Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS budgets (
            budget_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            initial_funds REAL NOT NULL,
            current_funds REAL NOT NULL
        );";
    db.execute(query, ())
}

pub fn insert_new_budget(db: &Connection, budget: &Budget) -> Result<usize, Error> {
    let query = "
        INSERT INTO budgets (name, initial_funds, current_funds)
        VALUES (?1, ?2, ?3);";
    db.execute(
        query,
        (&budget.name, &budget.initial_funds, &budget.current_funds),
    )
}

pub fn get_budget_by_id(db: &Connection, id: &u32) -> Result<Budget, Error> {
    let query = "
        SELECT *
        FROM budgets
        WHERE budget_id = ?1;";
    db.query_row(query, [id], |row| {
        Ok(Budget {
            budget_id: row.get(0)?,
            name: row.get(1)?,
            initial_funds: row.get(2)?,
            current_funds: row.get(3)?,
        })
    })
}

pub fn get_all_budgets(db: &Connection) -> Result<Vec<Budget>, Error> {
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
    let mut budget_list = Vec::new();
    for budget in budget_iter {
        budget_list.push(budget?);
    }
    Ok(budget_list)
}

pub fn update_budget(db: &Connection, budget: &Budget) -> Result<usize, Error> {
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
            &budget.budget_id.unwrap(),
        ),
    )
}

pub fn delete_budget_by_id(db: &Connection, id: &u32) -> Result<usize, Error> {
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
        let mut budget = Budget::new("Test", 5000.0);
        budget.reduce_funds(&3000.0);
        assert_eq!(budget.current_funds, 2000.0);
    }

    #[test]
    fn reduce_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.reduce_funds(&3000.0);
        assert_ne!(budget.current_funds, 3000.0);
    }

    #[test]
    fn increase_funds_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(&3000.0);
        assert_eq!(budget.current_funds, 8000.0);
    }

    #[test]
    fn increase_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(&3000.0);
        assert_ne!(budget.current_funds, 7000.0);
    }

    #[test]
    fn reset_funds_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(&3000.0);
        budget.reset_funds();
        assert_eq!(budget.current_funds, budget.initial_funds);
    }

    #[test]
    fn reset_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(&3000.0);
        budget.reset_funds();
        assert_ne!(budget.current_funds, 2000.0);
    }

    #[test]
    fn set_funds_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.set_current_funds(&3000.0);
        assert_eq!(budget.current_funds, 3000.0);
    }

    #[test]
    fn set_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.set_current_funds(&3000.0);
        assert_ne!(budget.current_funds, 5000.0);
    }

    #[test]
    fn rename_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.rename("New name");
        assert_eq!(budget.name, "New name");
    }

    #[test]
    fn rename_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.rename("New name");
        assert_ne!(budget.name, String::from("Test".to_string()));
    }
}
