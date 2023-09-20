use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Record {
    record_id: i32,
    value: f64,
    description: Option<String>,
    entity: String,
    category: String,
}

fn main() -> Result<()> {
    let connection = Connection::open_in_memory()?;

    connection.execute(
        "CREATE TABLE records (
            id INTEGER PRIMARY KEY,
            value REAL,
            description TEXT,
            entity TEXT,
            category TEXT
        )",
        (),
    )?;

    let data = Record {
        record_id: 1,
        value: 1000.0,
        description: Some(String::from("Para el mercado")),
        entity: String::from("Nequi"),
        category: String::from("Gastos"),
    };

    connection.execute(
        "INSERT INTO records VALUES(?1, ?2, ?3, ?4, ?5)",
        (
            &data.record_id,
            &data.value,
            &data.description,
            &data.entity,
            &data.category,
        ),
    )?;

    let mut stmt = connection.prepare("SELECT * FROM records")?;
    let record_iter = stmt.query_map([], |row| {
        Ok(Record {
            record_id: row.get(0)?,
            value: row.get(1)?,
            description: row.get(2)?,
            entity: row.get(3)?,
            category: row.get(4)?,
        })
    })?;

    for record in record_iter {
        println!("Found record {:?}", record.unwrap());
    }

    Ok(())
}
