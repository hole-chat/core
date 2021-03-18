use super::types::Message;

use rusqlite::{params, Connection, Result};

pub fn select_message_by_id(id: u64, conn: &Connection) -> Result<Message> {
    unimplemented!();
}

pub fn select_all_user_message(id: u64, conn: &Connection) -> Result<Vec<Message>> {
    unimplemented!();
}

pub fn select_n_last_messages(user_id: u32, start: u32, count: u32,  conn: &Connection) -> Result<Vec<Message>> {
    let mut selected = conn.prepare("SELECT * FROM messages WHERE user_id = ?1 AND id > ?2 LIMIT ?3")?;
    let message_iter = selected.query_map(params![user_id, start, count], |row| {
        Ok(Message {
            id: row.get(0)?,
            date: row.get(1)?,
            user_id: row.get(2)?,
            message: row.get(3)?,
        })
    })?;
    let mut users: Vec<Message> = Vec::new();
    for message in message_iter {
        log::info!("User: {:?}", (&message));
        users.push(message?);
    }
    log::info!("All users loaded to memory");
    Ok(users)
}

pub fn add_message(message: Message, conn: &Connection) -> Result<()> {
    match conn.execute(
        "INSERT INTO messages (
                  id,
                  user_id,
                  date,
                  message
                  ) VALUES (?1, ?2, ?3, ?4)",
        params![message.id,message.user_id, message.date, message.message],
    ) {
        Ok(_) => log::info!("message {:} added succsessfully!", message.id),
        Err(e) => {
            log::error!("failed to insert message {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}
