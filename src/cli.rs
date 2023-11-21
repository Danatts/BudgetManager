use crate::budget::{
    delete_budget_by_id, get_all_budgets, get_budget_by_id, insert_new_budget, print_all_budgets,
    print_budget, update_budget, Budget,
};
use crate::database::open_db;
use std::io::Write;

// TODO: Refactorizar para usar enum
// enum BudgetAction {
//     List,
//     New,
//     Reset,
//     Delete,
// }

fn prompt() -> String {
    let mut line = String::new();
    print!("{}", ">> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Could not read a line.");

    return line.trim().to_string();
}

// TODO:
// 1. añadir opción de ayuda
// 2. implementar elección de la base de datos depende del entorno
pub fn run() {
    let db = open_db("./database.test.db3");
    println!("\nType 'help' for more information.");
    loop {
        println!("\nChoose an action:");
        let input = prompt();
        if input == "list" {
            let budgets = get_all_budgets(&db);
            match budgets {
                Ok(budgets) => print_all_budgets(budgets),
                Err(error) => eprintln!("Error: {}.", error),
            }
        } else if input == "new" {
            println!("New budget name:");
            let name = prompt();
            println!("Budget inital funds:");
            let funds = prompt().parse::<f64>();
            if let Ok(funds) = funds {
                let budget = Budget::new(name, funds);
                match insert_new_budget(&db, &budget) {
                    Ok(rows) => println!("{} record inserted.", rows),
                    Err(error) => eprintln!("Error: {}", error),
                }
            } else {
                eprintln!("Enter a valid input.");
            }
        } else if input == "reset" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                match get_budget_by_id(&db, id) {
                    Ok(mut budget) => {
                        budget.reset_funds();
                        match update_budget(&db, &budget) {
                            Ok(rows) => {
                                println!("{} record updated.", rows);
                                print_budget(&budget);
                            }
                            Err(error) => eprintln!("Error: {}", error),
                        };
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "reduce" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                match get_budget_by_id(&db, id) {
                    Ok(mut budget) => {
                        println!("Amount:");
                        let amount = prompt().parse::<f64>();
                        if let Ok(amount) = amount {
                            budget.reduce_funds(amount);
                            match update_budget(&db, &budget) {
                                Ok(rows) => {
                                    println!("{} record updated.", rows);
                                    print_budget(&budget);
                                }
                                Err(error) => eprintln!("Error: {}", error),
                            };
                        } else {
                            println!("Enter a valid input.");
                        }
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "increase" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                match get_budget_by_id(&db, id) {
                    Ok(mut budget) => {
                        println!("Amount:");
                        let amount = prompt().parse::<f64>();
                        if let Ok(amount) = amount {
                            budget.increase_funds(amount);
                            match update_budget(&db, &budget) {
                                Ok(rows) => {
                                    println!("{} record updated.", rows);
                                    print_budget(&budget);
                                }
                                Err(error) => eprintln!("Error: {}", error),
                            };
                        } else {
                            println!("Enter a valid input.");
                        }
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "delete" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                match delete_budget_by_id(&db, id) {
                    Ok(rows) => println!("{} record deleted.", rows),
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
        } else if input == "rename" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                match get_budget_by_id(&db, id) {
                    Ok(mut budget) => {
                        println!("New name:");
                        let new_name = prompt();
                        budget.rename(new_name);
                        match update_budget(&db, &budget) {
                            Ok(rows) => {
                                println!("{} record updated.", rows);
                                print_budget(&budget);
                            }
                            Err(error) => eprintln!("Error: {}", error),
                        };
                    }
                    Err(error) => eprintln!("Error: {}", error),
                }
            }
        } else if input == "exit" {
            break;
        } else {
            println!("Enter a valid option: type 'help' for more information");
        }
    }
}
