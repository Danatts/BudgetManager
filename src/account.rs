use crate::cli::Args;

#[derive(Debug)]
pub struct Account {
    pub account_id: u32,
    pub value: f64,
    pub entity: String,
    pub category: String,
}

impl Account {
    pub fn build(args: &Args) -> Account {
        Account {
            account_id: 1,
            value: args.value,
            entity: args.entity.clone(),
            category: args.category.clone(),
        }
    }
}
