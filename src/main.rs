use budget::{get_all_budgets, reset_all_budgets};
use database::open_db;

pub mod budget;
pub mod database;
pub mod transaction;

// use budget::Budget;

fn main() {
    let db = open_db("./database.test.db3");
    let mut budgets = get_all_budgets(&db).unwrap();
    reset_all_budgets(&mut budgets);
    for budget in budgets {
        println!("{budget}");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn main_ok() {}
}
