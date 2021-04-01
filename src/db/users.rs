use super::types::User;
use fcpv2::types::{SSK, traits::FcpParser};

use rusqlite::{params, Connection, Result, NO_PARAMS};

pub fn get_user_by_id(id: crate::db::types::Id, conn: &Connection) -> Result<User> {
    let mut selected = conn.prepare("SELECT * FROM users WHERE id = ?")?;
    let mut user_iter = selected.query_map(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            sign_key: row.get(2)?,
            insert_key: row.get(3)?,
            messages_count: row.get(4)?,
        })
    })?;
    let user = user_iter.next().unwrap();
    log::info!("User {:} founded", id.0.to_string());
    user
}

pub fn load_all_users(conn: &Connection) -> Result<Vec<User>> {
    let mut selected = conn.prepare("SELECT * FROM users")?;
    let user_iter = selected.query_map(params![], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            sign_key: row.get(2)?,
            insert_key: row.get(3)?,
            messages_count: row.get(4)?,
        })
    })?;
    let mut users: Vec<User> = Vec::new();
    for user in user_iter{
        log::info!("User: {:?}", (&user));
        users.push(user?);
    }
    log::info!("All users loaded to memory");
    Ok(users)
}

pub fn add_user(user: User, conn: &Connection) -> Result<()> {
    match conn.execute(
        "INSERT INTO users (
                  id,
                  name,
                  sign_key,
                  insert_key,
                  messages_count
                  ) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            user.id,
            user.name,
            user.sign_key,
            user.insert_key,
            user.messages_count
        ],
    ) {
        Ok(_) => log::info!("add user to USERS successfully!"),
        Err(e) => {
            log::error!("failed to add user {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}
