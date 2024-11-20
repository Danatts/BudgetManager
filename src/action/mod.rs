use rusqlite::{params, Connection, Result};

#[derive(Debug)]
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

    pub fn insert_action(&self, conn: &Connection) -> Result<u32> {
        let query = "
            INSERT INTO actions (name, amount, old_value, new_value)
            VALUES (?1, ?2, ?3, ?4);
            ";
        conn.execute(
            query,
            params![&self.name, &self.amount, &self.old_value, &self.new_value,],
        )?;
        let id = conn.last_insert_rowid();
        Ok(id as u32)
    }

    pub fn get_action_by_id(conn: &Connection, action_id: u32) -> Result<Action> {
        let query = "
            SELECT action_id, name, amount, old_value, new_value
            FROM actions
            WHERE action_id = ?1
            ";

        conn.query_row(query, params![action_id], |row| {
            Ok(Action {
                action_id: row.get(0)?,
                name: row.get(1)?,
                amount: row.get(2)?,
                old_value: row.get(3)?,
                new_value: row.get(4)?,
            })
        })
    }
}

pub fn create_action_table(conn: &Connection) -> Result<()> {
    let query = "
        CREATE TABLE IF NOT EXISTS actions (
            action_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            amount REAL NOT NULL,
            old_value REAL NOT NULL,
            new_value REAL NOT NULL
        );";
    conn.execute(query, [])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory().unwrap();
        create_action_table(&conn).unwrap();
        Ok(conn)
    }

    #[test]
    fn get_action_by_id_ok() {
        let conn = setup_test_db().unwrap();

        let action = Action::new("test_action", Some(100.0), 500.0, 600.0);
        action.insert_action(&conn).unwrap();

        let action = Action::get_action_by_id(&conn, 1).unwrap();

        assert_eq!(action.action_id, Some(1));
        assert_eq!(action.name, "test_action");
        assert_eq!(action.amount, 100.0);
        assert_eq!(action.old_value, 500.0);
        assert_eq!(action.new_value, 600.0);
    }

    #[test]
    fn get_action_by_id_ko() {
        let conn = setup_test_db().unwrap();

        let action = Action::get_action_by_id(&conn, 1);

        assert!(action.is_err_and(|x| x == rusqlite::Error::QueryReturnedNoRows));
    }

    #[test]
    fn insert_action_ok() {
        let conn = setup_test_db().unwrap();

        let action = Action::new("test_action", Some(100.0), 500.0, 600.0);
        let id = action.insert_action(&conn).unwrap();

        assert_eq!(id, 1);
    }
}
