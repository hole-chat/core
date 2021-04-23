use rusqlite::{params, Connection, Result};
pub mod messages;
pub mod types;
pub mod users;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use core::marker::Send;
use r2d2_sqlite::SqliteConnectionManager;

pub use r2d2::{ManageConnection, Pool};

use types::DB_PATH;

fn create_db(conn: &Connection) -> Result<()> {
    match conn.execute(
        "CREATE TABLE users (
                  id                   BLOB PRIMARY KEY,
                  name                 TEXT UNIQUE NOT NULL,
                  sign_key             BLOB UNIQUE NOT NULL,
                  insert_key           BLOB UNIQUE NOT NULL,
                  messages_count       INTEGER,
                  my_messages_count    INTEGER
                  )",
        params![],
    ) {
        Ok(_) => log::info!("USER table created successfully!"),
        Err(e) => log::error!("failed to create USER table {:?}", e),
    }
    match conn.execute(
        "CREATE TABLE messages (
                  id                      INTEGER PRIMARY KEY,
                  user_id                 BLOB NOT NULL,
                  date                    datetime NOT NULL,
                  message                 TEXT NOT NULL,
                  from_me              BOOL
                  )",
        params![],
    ) {
        Ok(_) => log::info!("MESSAGES table created successfully!"),
        Err(e) => log::error!("failed to create USER table {:?}", e),
    }
    match conn.execute(
        "CREATE TABLE my_messages (
                  id                      INTEGER PRIMARY KEY,
                  user_id                 BLOB NOT NULL,
                  date                    datetime NOT NULL,
                  message                 TEXT NOT NULL,
                  from_me              BOOL
                  )",
        params![],
    ) {
        Ok(_) => log::info!("MY_MESSAGES table created successfully!"),
        Err(e) => log::error!("failed to create USER table {:?}", e),
    }
    Ok(())
}

pub fn start_db() -> std::result::Result<Pool<SqliteConnectionManager>, r2d2::Error> where
{
    let manager = r2d2_sqlite::SqliteConnectionManager::file(DB_PATH);

    if !std::path::Path::new(DB_PATH).exists() {
        let conn = Connection::open(DB_PATH).expect("failed to connect to db");
        println!("{}", conn.is_autocommit());
        create_db(&conn).expect("failed to create db");
        Ok(r2d2::Pool::new(manager).unwrap())
    } else {
        Ok(r2d2::Pool::new(manager).unwrap())
    }
}

pub fn old_start_db() -> Result<Connection> {
    if !std::path::Path::new(DB_PATH).exists() {
        let conn = Connection::open(DB_PATH)?;
        println!("{}", conn.is_autocommit());
        match create_db(&conn) {
            Ok(_) => {
                log::info!("Successfully created DB!");
                Ok(conn)
            }
            Err(e) => {
                log::error!("Failed to create DB: {:?}", e);
                Err(e)
            }
        }
    } else {
        Connection::open(DB_PATH)
    }
}

// ### A little stupid test ### //
//let conn = db::start_db().unwrap();
/*
 users::add_user(db::types::User{
     id: 9349,
     name: "Nick".to_string(),
     sign_key: "string".to_string(),
     insert_key: fcpv2::types::SSK::parse("SSK@Rgt0qM8D24DltliV2-JE9tYLcrgGAKeDwkz41I3JBPs,p~c8c7FXcJjhcf2vA-Xm0Mjyw1o~xn7L2-T8zlBA1IU").unwrap(),
     messages_count: 1,
 }, &conn);
 let time: chrono::DateTime<chrono::offset::FixedOffset> =
     chrono::DateTime::parse_from_rfc3339("2021-03-18T04:22:42.501Z").unwrap();
 db::messages::add_message(
     db::types::Message {
         user_id: 9349,
         id: 4,
         date: time.naive_utc(),
         message: "HI?".to_string(),
     },
     &conn,
 )
 .unwrap();
 db::messages::add_message(
     db::types::Message {
         user_id: 9349,
         id: 5,
         date: time.naive_utc(),
         message: "I AM NICK!".to_string(),
     },
     &conn,
 )
 .unwrap();
 db::messages::add_message(
     db::types::Message {
         user_id: 9349,
         id: 6,
         date: time.naive_utc(),
         message: "I'LL FIND that".to_string(),
     },
     &conn,
 )
 .unwrap();

 let messages = db::messages::select_message_by_id(9349, 3, &conn).unwrap();
*/
