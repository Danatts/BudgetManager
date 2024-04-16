use crate::budget::create_budget_table;
use crate::services::{
    create_budget, get_budgets, get_history, increase_funds, reduce_funds, remove_budget,
    rename_budget, reset_funds, set_current_funds, set_initial_funds,
};
use crate::transaction::create_transaction_table;
use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::path::PathBuf;
use std::process;

#[derive(Subcommand)]
pub enum Command {
    /// Set current budget funds
    Current {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
        /// Add small description
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
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
        /// Add small description
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Increase budget funds
    Increase {
        #[arg(value_name = "ID")]
        id: u32,
        #[arg(value_name = "AMOUNT")]
        amount: f64,
        /// Add small description
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
        /// Add small description
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
    /// Remove a budget
    Remove {
        #[arg(value_name = "ID")]
        id: u32,
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
        /// Add small description
        #[arg(long, short, value_name = "DESCRIPTION")]
        description: Option<String>,
    },
}

impl Command {
    pub fn value(&self) -> &str {
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
    /// Select a database file
    #[arg(long, short, value_name = "FILE NAME")]
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
        } => set_current_funds(&db, id, amount, &command, description),
        Command::History { id } => get_history(&db, id),
        Command::Increase {
            id,
            amount,
            description,
        } => increase_funds(&db, id, amount, &command, description),
        Command::Initial {
            id,
            amount,
            description,
        } => set_initial_funds(&db, id, amount, &command, description),
        Command::List => get_budgets(&db),
        Command::New { name, funds } => create_budget(&db, name, funds),
        Command::Reduce {
            id,
            amount,
            description,
        } => reduce_funds(&db, id, amount, &command, description),
        Command::Remove { id } => remove_budget(&db, id),
        Command::Rename { id, name } => rename_budget(&db, id, name),
        Command::Reset { id, description } => reset_funds(&db, id, &command, description),
    }
}
