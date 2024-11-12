use rusqlite::Connection;

pub struct Action {
    pub action_id: Option<u32>,
    pub name: String,
    pub amount: f64,
    pub old_value: f64,
    pub new_value: f64,
}

impl Action {
    pub fn new(name: &str, amount: Option<f64>, old_value: f64, new_value: f64) -> Self {
        Self {
            action_id: None,
            name: name.to_string(),
            amount: amount.unwrap_or(0.0),
            old_value,
            new_value,
        }
    }
}

pub fn create_action_table(db: &Connection) -> Result<usize, rusqlite::Error> {
    let query = "
        CREATE TABLE IF NOT EXISTS actions (
            action_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            amount REAL NOT NULL,
            old_value REAL NOT NULL,
            new_value REAL NOT NULL
        );";
    db.execute(query, ())
}

pub fn insert_action(db: &Connection, action: &Action) -> Result<usize, rusqlite::Error> {
    let query = "
        INSERT INTO actions (name, amount, old_value, new_value)
        VALUES (?1, ?2, ?3, ?4);";
    db.execute(
        query,
        (
            &action.name,
            &action.amount,
            &action.old_value,
            &action.new_value,
        ),
    )
}
