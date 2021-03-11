use rusqlite::{params, Connection, Result};
pub mod messaging;

#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    key: String,
}

fn create_db(conn: &Connection) -> Result<()> {
        match conn.execute(
            "CREATE TABLE  users (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  key             BLOB 
                  )",
            params![],
        ) {
            Ok(_) => {log::info!("USER table created successfully!")},
            Err(e) => log::error!("failed to create USER table {:?}", e),
        }
    Ok(())
}


pub fn start_db() -> Result<()> {
    let conn = Connection::open("hole.db")?;
    println!("{}", conn.is_autocommit());
    match create_db(&conn) {
        Ok(_) => log::info!("Successfully created DB!"),
        Err(e) => log::error!("Failed to create DB: {:?}", e)
    }
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        key: "SSK@OolaRmEpOc1q0JF9iypUHZTlNNIqstOnScyb15SUr6k,MgxYrnex5LfvW-pRwMINs~d4nE2mYKjW1AE1U9vIPUM,AQECAAE".to_string(),
    };
    conn.execute(
        "INSERT INTO users (name, key) VALUES (?1, ?2)",
        params![me.name, me.key],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, key FROM users")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            key: row.get(2)?,
        })
    })?;


    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}
