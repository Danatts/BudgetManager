pub mod budget;
pub mod database;
pub mod transaction;

use budget::Budget;

fn main() {
    let db = database::open_db("./database.test.db3").expect("could not connect to database");
    database::create_budget_table(&db);

    let mut budget = Budget::new("Mercado", 500.0);
    database::insert_new_budget(&db, &budget);
    println!("{:?}", budget);
    budget.budget_id = Some(1);
    budget.name = "Libre".to_string();
    database::update_budget(&db, &budget);
    println!("{:?}", budget);
}

#[cfg(test)]
mod tests {

    #[test]
    fn main_ok() {}
}
