use rusqlite::{params, Connection, Result};
pub mod types;
pub mod users;
pub mod messages;

use types::DB_PATH;

fn create_db(conn: &Connection) -> Result<()> {
    match conn.execute(
        "CREATE TABLE users (
                  id                   INTEGER PRIMARY KEY,
                  name                 TEXT UNIQUE NOT NULL,
                  sign_key             BLOB NOT NULL,
                  insert_key           BLOB NOT NULL,
                  messages_count       INTEGER
                  )",
        params![],
    ) {
        Ok(_) => log::info!("USER table created successfully!"),
        Err(e) => log::error!("failed to create USER table {:?}", e),
    }
    match conn.execute(
        "CREATE TABLE messages (
                  id                      INTEGER PRIMARY KEY,
                  user_id                 TEXT NOT NULL,
                  date                    datetime NOT NULL,
                  message                 TEXT NOT NULL
                  )",
        params![],
    ) {
        Ok(_) => log::info!("MESSAGES table created successfully!"),
        Err(e) => log::error!("failed to create USER table {:?}", e),
    }
    Ok(())
}

pub fn start_db() -> Result<Connection> {
    if !std::path::Path::new(DB_PATH).exists() {
        let conn = Connection::open(DB_PATH)?;
        println!("{}", conn.is_autocommit());
        match create_db(&conn) {
            Ok(_) => {log::info!("Successfully created DB!"); Ok(conn)},
            Err(e) => {log::error!("Failed to create DB: {:?}",e ); Err(e)},
        }
    } else {
        Connection::open(DB_PATH)
    }
}
