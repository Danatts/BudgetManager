use crate::budget::{
    delete_budget_by_id, insert_budget, select_all_budgets, select_budget_by_id, update_budget,
    Budget,
};
use crate::cli::Command;
use crate::record::{get_all_records, get_records_by_budget, insert_record, Record};
use rusqlite::{Connection, Error};

pub fn create_budget(db: &Connection, name: &str, funds: &f64) -> Result<usize, Error> {
    let budget = Budget::new(name, funds);
    insert_budget(db, &budget)
}

pub fn remove_budget(db: &Connection, id: &u32) -> Result<usize, Error> {
    delete_budget_by_id(db, id)
}

pub fn rename_budget(db: &Connection, id: &u32, name: &str) -> Result<usize, Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let budget = &mut budgets[0];
    budget.rename(name);
    update_budget(db, budget)
}

pub fn increase_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let budget = &mut budgets[0];
    budget.increase_funds(amount);
    let rows = update_budget(db, budget)?;
    let transaction = Record::new(id, command.value(), amount, description);
    insert_record(db, &transaction)?;
    Ok(rows)
}

pub fn reduce_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let budget = &mut budgets[0];
    budget.reduce_funds(amount);
    let rows = update_budget(db, budget)?;
    let transaction = Record::new(id, command.value(), amount, description);
    insert_record(db, &transaction)?;
    Ok(rows)
}

pub fn reset_funds(
    db: &Connection,
    id: &u32,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let budget = &mut budgets[0];
    budget.reset_funds();
    let rows = update_budget(db, budget)?;
    let transaction = Record::new(id, command.value(), &budget.initial_funds, description);
    insert_record(db, &transaction)?;
    Ok(rows)
}

pub fn set_current_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let budget = &mut budgets[0];
    budget.set_current_funds(amount);
    let rows = update_budget(db, budget)?;
    let transaction = Record::new(id, command.value(), amount, description);
    insert_record(db, &transaction)?;
    Ok(rows)
}

pub fn set_initial_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let budget = &mut budgets[0];
    budget.set_initial_funds(amount);
    let rows = update_budget(db, budget)?;
    let transaction = Record::new(id, command.value(), amount, description);
    insert_record(db, &transaction)?;
    Ok(rows)
}

pub fn get_budgets(db: &Connection) -> Result<Vec<Budget>, Error> {
    select_all_budgets(db)
}

pub fn get_history(db: &Connection, id: &Option<u32>) -> Result<Vec<Record>, Error> {
    match id {
        Some(id) => get_records_by_budget(db, id),
        None => get_all_records(db),
    }
}
