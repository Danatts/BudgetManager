pub mod budget;
pub mod database;
pub mod transaction;

fn main() {
    let db = database::open_db("./database.db3").expect("could not connect to database");
    database::create_budget_table(&db);
}
