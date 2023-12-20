use crate::budget::{
    create_budget_table, delete_budget_by_id, get_all_budgets, get_budget_by_id, insert_new_budget,
    print_all_budgets, print_budget, update_budget, Budget,
};
use crate::transaction::{create_transaction_table, insert_transaction, Transaction};
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Subcommand)]
pub enum Commands {
    /// Set current budget funds
    Current {
        #[arg(short, value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
    },
    /// Delete a budget
    Delete {
        #[arg(value_name = "ID")]
        id: u32,
    },
    /// Set initial budget funds
    Initial {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
    },
    /// Increase budget funds
    Increase {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
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
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub fn cli(db: &Connection) {
    match create_budget_table(&db) {
        Ok(_) => {}
        Err(error) => panic!("Error: {}.", error),
    }

    match create_transaction_table(&db) {
        Ok(_) => {}
        Err(error) => panic!("Error: {}.", error),
    }

    let cli = Cli::parse();

    let action = match &cli.command {
        Commands::Current { id: _, amount: _ } => "Set current funds",
        Commands::Increase { id: _, amount: _ } => "Increase funds",
        Commands::Initial { id: _, amount: _ } => "Set initial funds",
        Commands::Reduce { id: _, amount: _ } => "Reduce funds",
        _ => "",
    };

    match &cli.command {
        Commands::Current { id, amount } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.set_current_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction = Transaction::new(id, action, amount);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Commands::Delete { id } => match delete_budget_by_id(&db, id) {
            Ok(rows) => println!("{} record deleted.", rows),
            Err(error) => eprintln!("Error: {}", error),
        },
        Commands::Increase { id, amount } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.increase_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction = Transaction::new(id, action, amount);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Commands::Initial { id, amount } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.set_initial_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction = Transaction::new(id, action, amount);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Commands::List => match get_all_budgets(&db) {
            Ok(budgets) => print_all_budgets(&budgets),
            Err(error) => eprintln!("Error: {}.", error),
        },
        Commands::New { name, funds } => {
            let budget = Budget::new(name, funds.to_owned());
            match insert_new_budget(&db, &budget) {
                Ok(rows) => {
                    println!("{} record inserted.", rows);
                    print_budget(&budget);
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Commands::Reduce { id, amount } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.reduce_funds(amount);
                match update_budget(&db, &budget) {
                    Ok(rows) => {
                        let transaction = Transaction::new(id, action, amount);
                        let _ = insert_transaction(&db, &transaction);
                        println!("{} record updated.", rows);
                        print_budget(&budget);
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        },
        Commands::Rename { id, name } => match get_budget_by_id(&db, id) {
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
        Commands::Reset { id } => match get_budget_by_id(&db, id) {
            Ok(mut budget) => {
                budget.reset_funds();
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
    }
}
