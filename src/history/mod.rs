use accounting::Accounting;
use chrono::{DateTime, Local};
use core::fmt;

pub struct History {
    pub date: DateTime<Local>,
    pub budget_name: String,
    pub action_name: String,
    pub value: f64,
    pub description: Option<String>,
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ac = Accounting::new_from_seperator("$", 2, ".", ",");
        let date = self.date.format("%d-%m-%Y");
        let desc = match &self.description {
            Some(desc) => desc.to_owned(),
            None => String::new(),
        };
        write!(
            f,
            "{:<15}{:<20}{:<20}{:<25}{:<25}",
            date,
            self.budget_name,
            self.action_name,
            ac.format_money(self.value),
            desc
        )
    }
}

pub fn print_history(history: &Vec<History>) {
    println!(
        "\n{:<15}{:<20}{:<20}{:<25}{:<25}\n{:-^110}",
        "DATE", "BUDGET", "ACTION", "VALUE", "DESCRIPTION", ""
    );
    for record in history {
        println!("{record}");
    }
}
