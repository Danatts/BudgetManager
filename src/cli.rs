use crate::budget::{
    delete_budget_by_id, get_budget_by_id, insert_new_budget, list_all_budgets, print_budget,
    update_budget, Budget,
};
use crate::database::open_db;
use std::io::Write;

// enum BudgetAction {
//     List,
//     New,
//     Reset,
//     Delete,
// }

pub fn prompt() -> String {
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
// 3. mejorar mensajes de error
pub fn run() {
    let db = open_db("./database.test.db3");
    println!("\nType 'help' for more information.");
    loop {
        println!("\nChoose an action:");
        let input = prompt();
        if input == "list" {
            list_all_budgets(&db);
        } else if input == "new" {
            println!("New budget name:");
            let name = prompt();
            println!("Budget inital funds:");
            let funds = prompt().parse::<f64>();
            if let Ok(funds) = funds {
                let budget = Budget::new(name, funds);
                insert_new_budget(&db, &budget);
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "reset" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                let budget = get_budget_by_id(&db, id);
                match budget {
                    Ok(mut budget) => {
                        budget.reset_funds();
                        update_budget(&db, &budget);
                        print_budget(&budget);
                    }
                    Err(_) => {}
                }
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "reduce" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                let budget = get_budget_by_id(&db, id);
                match budget {
                    Ok(mut budget) => {
                        println!("Amount:");
                        let amount = prompt().parse::<f64>();
                        if let Ok(amount) = amount {
                            budget.reduce_funds(amount);
                            update_budget(&db, &budget);
                            print_budget(&budget);
                        } else {
                            println!("Enter a valid input.");
                        }
                    }
                    Err(_) => {}
                }
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "increase" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                let budget = get_budget_by_id(&db, id);
                match budget {
                    Ok(mut budget) => {
                        println!("Amount:");
                        let amount = prompt().parse::<f64>();
                        if let Ok(amount) = amount {
                            budget.increase_funds(amount);
                            update_budget(&db, &budget);
                            print_budget(&budget);
                        } else {
                            println!("Enter a valid input.");
                        }
                    }
                    Err(_) => {}
                }
            } else {
                println!("Enter a valid input.");
            }
        } else if input == "delete" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                delete_budget_by_id(&db, id);
            }
        } else if input == "rename" {
            println!("Budget id:");
            let id = prompt().parse::<u32>();
            if let Ok(id) = id {
                let budget = get_budget_by_id(&db, id);
                match budget {
                    Ok(mut budget) => {
                        println!("New name:");
                        let new_name = prompt();
                        budget.rename(new_name);
                        update_budget(&db, &budget);
                        print_budget(&budget);
                    }
                    Err(_) => {}
                }
            }
        } else if input == "exit" {
            break;
        } else {
            println!("Enter a valid option: type 'help' for more information");
        }
    }
}
