#[derive(Debug)]
pub struct Budget {
    pub budget_id: Option<u32>,
    pub name: String,
    pub initial_funds: f64,
    pub current_funds: f64,
}

impl Budget {
    pub fn new(name: &str, funds: f64) -> Budget {
        Budget {
            budget_id: None,
            name: name.to_string(),
            initial_funds: funds,
            current_funds: funds,
        }
    }

    pub fn increase_funds(&mut self, amount_to_increase: f64) {
        self.current_funds += amount_to_increase;
    }

    pub fn reduce_funds(&mut self, amount_to_reduce: f64) {
        self.current_funds -= amount_to_reduce;
    }

    pub fn reset_funds(&mut self) {
        self.current_funds = self.initial_funds;
    }

    pub fn set_current_funds(&mut self, amount_to_set: f64) {
        self.current_funds = amount_to_set;
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
        let mut budget = Budget::new("Test", 5000.0);
        budget.reduce_funds(3000.0);
        assert_eq!(budget.current_funds, 2000.0);
    }

    #[test]
    fn reduce_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.reduce_funds(3000.0);
        assert_ne!(budget.current_funds, 3000.0);
    }

    #[test]
    fn increase_funds_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(3000.0);
        assert_eq!(budget.current_funds, 8000.0);
    }

    #[test]
    fn increase_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(3000.0);
        assert_ne!(budget.current_funds, 7000.0);
    }

    #[test]
    fn reset_funds_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(3000.0);
        budget.reset_funds();
        assert_eq!(budget.current_funds, budget.initial_funds);
    }

    #[test]
    fn reset_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.increase_funds(3000.0);
        budget.reset_funds();
        assert_ne!(budget.current_funds, 2000.0);
    }

    #[test]
    fn set_funds_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.set_current_funds(3000.0);
        assert_eq!(budget.current_funds, 3000.0);
    }

    #[test]
    fn set_funds_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.set_current_funds(3000.0);
        assert_ne!(budget.current_funds, 5000.0);
    }

    #[test]
    fn rename_ok() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.rename("New name");
        assert_eq!(budget.name, "New name");
    }

    #[test]
    fn rename_ko() {
        let mut budget = Budget::new("Test", 5000.0);
        budget.rename("New name");
        assert_ne!(budget.name, String::from("Test"));
    }
}
