use crate::action::{insert_action, Action};
use crate::budget::{
    delete_budget_by_id, insert_budget, select_all_budgets, select_budget_by_id, update_budget,
    Budget,
};
use crate::cli::Command;
use crate::history::History;
use crate::record::{get_all_records, get_records_by_budget, insert_record, Record};
use rusqlite::Connection;

pub fn create_budget(db: &Connection, name: &str, funds: &f64) -> Result<usize, rusqlite::Error> {
    let budget = Budget::new(name, funds);
    insert_budget(db, &budget)
}

pub fn remove_budget(db: &Connection, id: &u32) -> Result<usize, rusqlite::Error> {
    delete_budget_by_id(db, id)
}

pub fn rename_budget(db: &Connection, id: &u32, name: &str) -> Result<usize, rusqlite::Error> {
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
) -> Result<usize, rusqlite::Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let mut budget = budgets.swap_remove(0);
    let old_value = budget.current_funds;
    budget.increase_funds(amount);
    let new_value = budget.current_funds;
    let action = Action::new(command.value(), Some(*amount), old_value, new_value);
    insert_action(db, &action)?;
    let action_id = db.last_insert_rowid();
    let record = Record::new(budget.budget_id.unwrap(), action_id as u32, description);
    let rows = update_budget(db, &budget)?;
    insert_record(db, &record)?;
    Ok(rows)
}

pub fn reduce_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, rusqlite::Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let mut budget = budgets.swap_remove(0);
    let old_value = budget.current_funds;
    budget.reduce_funds(amount);
    let new_value = budget.current_funds;
    let action = Action::new(command.value(), Some(*amount), old_value, new_value);
    insert_action(db, &action)?;
    let rows = update_budget(db, &budget)?;
    let action_id = db.last_insert_rowid();
    let record = Record::new(budget.budget_id.unwrap(), action_id as u32, description);
    insert_record(db, &record)?;
    Ok(rows)
}

pub fn reset_funds(
    db: &Connection,
    id: &u32,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, rusqlite::Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let mut budget = budgets.swap_remove(0);
    let old_value = budget.current_funds;
    budget.reset_funds();
    let new_value = budget.current_funds;
    let action = Action::new(command.value(), None, old_value, new_value);
    insert_action(db, &action)?;
    let rows = update_budget(db, &budget)?;
    let action_id = db.last_insert_rowid();
    let record = Record::new(budget.budget_id.unwrap(), action_id as u32, description);
    insert_record(db, &record)?;
    Ok(rows)
}

pub fn set_current_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, rusqlite::Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let mut budget = budgets.swap_remove(0);
    let old_value = budget.current_funds;
    budget.set_current_funds(amount);
    let new_value = budget.current_funds;
    let action = Action::new(command.value(), Some(*amount), old_value, new_value);
    insert_action(db, &action)?;
    let rows = update_budget(db, &budget)?;
    let action_id = db.last_insert_rowid();
    let record = Record::new(budget.budget_id.unwrap(), action_id as u32, description);
    insert_record(db, &record)?;
    Ok(rows)
}

pub fn set_initial_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) -> Result<usize, rusqlite::Error> {
    let mut budgets = select_budget_by_id(db, id)?;
    let mut budget = budgets.swap_remove(0);
    let old_value = budget.current_funds;
    budget.set_initial_funds(amount);
    let new_value = budget.current_funds;
    let action = Action::new(command.value(), Some(*amount), old_value, new_value);
    insert_action(db, &action)?;
    let rows = update_budget(db, &budget)?;
    let action_id = db.last_insert_rowid();
    let record = Record::new(budget.budget_id.unwrap(), action_id as u32, description);
    insert_record(db, &record)?;
    Ok(rows)
}

pub fn get_budgets(db: &Connection) -> Result<Vec<Budget>, rusqlite::Error> {
    select_all_budgets(db)
}

pub fn get_history(db: &Connection, id: &Option<u32>) -> Result<Vec<History>, rusqlite::Error> {
    match id {
        Some(id) => get_records_by_budget(db, id),
        None => get_all_records(db),
    }
}
