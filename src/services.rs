use crate::budget::{
    delete_budget_by_id, select_all_budgets, select_budget_by_id, insert_budget, print_budgets,
    update_budget, Budget,
};
use crate::cli::Command;
use crate::transaction::{
    get_all_transactions, get_transactions_by_budget, insert_transaction, print_transactions,
    Transaction,
};
use rusqlite::Connection;

pub fn create_budget(db: &Connection, name: &str, funds: &f64) {
    let budget = Budget::new(name, funds);
    match insert_budget(db, &budget) {
        Ok(rows) => {
            println!("{} record inserted.", rows);
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn remove_budget(db: &Connection, id: &u32) {
    match delete_budget_by_id(db, id) {
        Ok(rows) => println!("{} record deleted.", rows),
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn rename_budget(db: &Connection, id: &u32, name: &str) {
    match select_budget_by_id(db, id) {
        Ok(mut budgets) => {
            let budget = &mut budgets[0];
            budget.rename(name);
            match update_budget(db, budget) {
                Ok(rows) => {
                    println!("{} record updated.", rows);
                    print_budgets(&budgets);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn increase_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) {
    match select_budget_by_id(db, id) {
        Ok(mut budgets) => {
            let budget = &mut budgets[0];
            budget.increase_funds(amount);
            match update_budget(db, budget) {
                Ok(rows) => {
                    let transaction = Transaction::new(id, command.value(), amount, description);
                    let _ = insert_transaction(db, &transaction);
                    println!("{} record updated.", rows);
                    print_budgets(&budgets);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn reduce_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) {
    match select_budget_by_id(db, id) {
        Ok(mut budgets) => {
            let budget = &mut budgets[0];
            budget.reduce_funds(amount);
            match update_budget(db, budget) {
                Ok(rows) => {
                    let transaction = Transaction::new(id, command.value(), amount, description);
                    let _ = insert_transaction(db, &transaction);
                    println!("{} record updated.", rows);
                    print_budgets(&budgets);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn reset_funds(db: &Connection, id: &u32, command: &Command, description: &Option<String>) {
    match select_budget_by_id(db, id) {
        Ok(mut budgets) => {
            let budget = &mut budgets[0];
            budget.reset_funds();
            match update_budget(db, budget) {
                Ok(rows) => {
                    let transaction =
                        Transaction::new(id, command.value(), &budget.initial_funds, description);
                    let _ = insert_transaction(db, &transaction);
                    println!("{} record updated.", rows);
                    print_budgets(&budgets);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn set_current_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) {
    match select_budget_by_id(db, id) {
        Ok(mut budgets) => {
            let budget = &mut budgets[0];
            budget.set_current_funds(amount);
            match update_budget(db, budget) {
                Ok(rows) => {
                    let transaction = Transaction::new(id, command.value(), amount, description);
                    let _ = insert_transaction(db, &transaction);
                    println!("{} record updated.", rows);
                    print_budgets(&budgets);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn set_initial_funds(
    db: &Connection,
    id: &u32,
    amount: &f64,
    command: &Command,
    description: &Option<String>,
) {
    match select_budget_by_id(db, id) {
        Ok(mut budgets) => {
            let budget = &mut budgets[0];
            budget.set_initial_funds(amount);
            match update_budget(db, budget) {
                Ok(rows) => {
                    let transaction = Transaction::new(id, command.value(), amount, description);
                    let _ = insert_transaction(db, &transaction);
                    println!("{} record updated.", rows);
                    print_budgets(&budgets);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => eprintln!("Error: {}", error),
    }
}

pub fn get_budgets(db: &Connection) {
    match select_all_budgets(db) {
        Ok(budgets) => print_budgets(&budgets),
        Err(error) => eprintln!("Error: {}.", error),
    }
}

pub fn get_history(db: &Connection, id: &Option<u32>) {
    match id {
        Some(id) => match get_transactions_by_budget(db, id) {
            Ok(list) => print_transactions(&list),
            Err(error) => eprintln!("Error: {}", error),
        },
        None => match get_all_transactions(db) {
            Ok(list) => print_transactions(&list),
            Err(error) => eprintln!("Error: {}", error),
        },
    }
}
