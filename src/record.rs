#[derive(Debug)]
pub struct Account {
    pub account_id: i32,
    pub value: f64,
    pub description: Option<String>,
    pub entity: String,
    pub category: String,
}

impl Account {
    pub fn build() -> Account {
        Account {
            account_id: 0,
            value: 1000.0,
            description: Some(String::from("Para el mercado")),
            entity: String::from("Nequi"),
            category: String::from("Gastos"),
        }
    }
}
