use std::path::PathBuf;
use std::process;

use crate::budget::{
    create_budget_table, delete_budget_by_id, get_all_budgets, get_budget_by_id, insert_new_budget,
    print_all_budgets, print_budget, update_budget, Budget,
};
use crate::transaction::{
    create_transaction_table, get_all_transactions, get_transactions_by_budget, insert_transaction,
    print_transactions, Transaction,
};
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Subcommand)]
pub enum Command {
    /// Set current budget funds
    Current {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Delete a budget
    Delete {
        #[arg(value_name = "ID")]
        id: u32,
    },
    /// Print transaction history
    History {
        #[arg(value_name = "ID")]
        id: Option<u32>,
    },
    /// Set initial budget funds
    Initial {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Increase budget funds
    Increase {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// List all budgets
    List,
    /// Create a new budget
    New {
        #[arg(value_name = "NAME")]
        name: String,
        #[arg(value_name = "FUNDS")]
        funds: f64,
    },
    /// Reduce budget funds
    Reduce {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Rename a budget
    Rename {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "NEW NAME")]
        name: String,
    },
    /// Reset a budget to initial funds
    Reset {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
}

impl Command {
    fn value(&self) -> &str {
        match self {
            Self::Current {
                id: _,
                amount: _,
                description: _,
            } => "Set current funds",
            Self::Increase {
                id: _,
                amount: _,
                description: _,
            } => "Increase funds",
            Self::Initial {
                id: _,
                amount: _,
                description: _,
            } => "Set initial funds",
            Self::Reduce {
                id: _,
                amount: _,
                description: _,
            } => "Reduce funds",
            Self::Reset {
                id: _,
                description: _,
            } => "Reset funds",
            _ => "",
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(long, short)]
    pub database: Option<PathBuf>,
}

pub fn run(db: Connection, command: Command) {
    match create_budget_table(&db) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {}.", error);
            process::exit(1);
        }
    }

    match create_transaction_table(&db) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error: {}.", error);
            process::exit(1);
        }
    }

    match &command {
        Command::Current {
            id,
            amount,
            description,
        } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.set_current_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction =
                            Transaction::new(id, command.value(), amount, description);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Command::Delete { id } => match delete_budget_by_id(&db, id) {
            Ok(rows) => println!("{} record deleted.", rows),
            Err(error) => eprintln!("Error: {}", error),
        },
        Command::History { id } => match id {
            Some(id) => match get_transactions_by_budget(&db, id) {
                Ok(list) => print_transactions(&list),
                Err(error) => eprintln!("Error: {}", error),
            },
            None => match get_all_transactions(&db) {
                Ok(list) => print_transactions(&list),
                Err(error) => eprintln!("Error: {}", error),
            },
        },
        Command::Increase {
            id,
            amount,
            description,
        } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.increase_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction =
                            Transaction::new(id, command.value(), amount, description);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Command::Initial {
            id,
            amount,
            description,
        } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.set_initial_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction =
                            Transaction::new(id, command.value(), amount, description);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Command::List => match get_all_budgets(&db) {
            Ok(budgets) => print_all_budgets(&budgets),
            Err(error) => eprintln!("Error: {}.", error),
        },
        Command::New { name, funds } => {
            let budget = Budget::new(name, funds.to_owned());
            match insert_new_budget(&db, &budget) {
                Ok(rows) => {
                    println!("{} record inserted.", rows);
                    print_budget(&budget);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Command::Reduce {
            id,
            amount,
            description,
        } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.reduce_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction =
                            Transaction::new(id, command.value(), amount, description);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Command::Rename { id, name } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.rename(name);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Command::Reset { id, description } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.reset_funds();
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction = Transaction::new(
                            id,
                            command.value(),
                            &budget.initial_funds,
                            description,
                        );
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
    }
}
