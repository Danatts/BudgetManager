#[derive(Debug)]
pub enum Action {
    Increase,
    Reduce,
    Set,
    Reset,
}

#[derive(Debug)]
pub struct Transaction {
    pub transaction_id: Option<u32>,
    pub budget_id: u32,
    pub action: Action,
    pub amount: f64,
    pub desc: String,
}

impl Transaction {
    pub fn new(budget_id: u32, action: Action, amount: f64, desc: &str) -> Transaction {
        Transaction {
            transaction_id: None,
            budget_id,
            action,
            amount,
            desc: desc.to_string(),
        }
    }
}
