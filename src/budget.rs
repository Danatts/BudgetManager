#[derive(Debug)]
pub struct Budget {
    pub budget_id: Option<i32>,
    pub name: String,
    pub funds: f64,
}

impl Budget {
    pub fn new(name: &str, amount: f64) -> Budget {
        Budget {
            budget_id: None,
            name: name.to_string(),
            funds: amount,
        }
    }

    pub fn increase_amount(&mut self, amount_to_increase: f64) {
        self.funds += amount_to_increase;
    }

    pub fn reduce_amount(&mut self, amount_to_reduce: f64) {
        if self.funds > amount_to_reduce {
            self.funds -= amount_to_reduce;
        } else {
            println!("Insufficient funds to reduce by {amount_to_reduce}");
        }
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_funds_ok() {
        let mut budget = Budget {
            budget_id: None,
            name: String::from("Test"),
            funds: 5000.0,
        };
        budget.reduce_amount(3000.0);
        assert_eq!(budget.funds, 2000.0);
    }

    #[test]
    fn reduce_funds_ko() {
        let mut budget = Budget {
            budget_id: None,
            name: String::from("Test"),
            funds: 5000.0,
        };
        budget.reduce_amount(3000.0);
        assert_ne!(budget.funds, 3000.0);
    }

    #[test]
    fn increase_funds_ok() {
        let mut budget = Budget {
            budget_id: None,
            name: String::from("Test"),
            funds: 5000.0,
        };
        budget.increase_amount(3000.0);
        assert_eq!(budget.funds, 8000.0);
    }

    #[test]
    fn increase_funds_ko() {
        let mut budget = Budget {
            budget_id: None,
            name: String::from("Test"),
            funds: 5000.0,
        };
        budget.increase_amount(3000.0);
        assert_ne!(budget.funds, 7000.0);
    }

    #[test]
    fn rename_ok() {
        let mut budget = Budget {
            budget_id: None,
            name: String::from("New name"),
            funds: 5000.0,
        };
        budget.rename("New name");
        assert_eq!(budget.name, "New name");
    }

    #[test]
    fn rename_ko() {
        let mut budget = Budget {
            budget_id: None,
            name: String::from("Test"),
            funds: 5000.0,
        };
        budget.rename("New name");
        assert_ne!(budget.name, String::from("Test"));
    }
}
