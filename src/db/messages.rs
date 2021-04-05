use super::types::Message;

use rusqlite::{params, Connection, Result};

fn ret_mes(row: &rusqlite::Row<'_>) -> Result<Message> {
    Ok(Message {
        id: row.get(0)?,
        user_id: row.get(1)?,
        date: row.get(2)?,
        message: row.get(3)?,
        from_me: row.get(4)?,
    })
}
type Id = crate::db::types::Id;

pub fn select_message_by_id(user_id: u32, id:u32, conn: &Connection) -> Result<Message> {
    let mut selected = conn.prepare("SELECT * FROM messages WHERE id = ?1 AND user_id = ?2")?;
    let mut message_iter = selected.query_map(params![id, user_id], |row| ret_mes(row))?;
    let message = message_iter.next().unwrap();
    log::info!("Message {:} founded", id);
    message
}

pub fn select_all_user_message(id: u32, conn: &Connection) -> Result<Vec<Message>> {
    let mut selected =
        conn.prepare("SELECT * FROM messages, my_messages WHERE user_id = ?1 ORDER BY date DESC")?;
    let message_iter = selected.query_map(params![id], |row| ret_mes(row))?;
    let mut messages: Vec<Message> = Vec::new();
    for message in message_iter {
        log::info!("Message: {:?}", (&message));
        messages.push(message?);
    }
    log::info!("All messages loaded to memory");
    Ok(messages)
}

pub fn select_n_last_messages(
    user_id: Id,
    start: u32,
    count: u32,
    conn: &Connection,
) -> Result<Vec<Message>> {
    let mut selected = conn.prepare(
        "SELECT * FROM messages WHERE user_id = ?1 AND id >= ?2 LIMIT ?3 ORDER BY date DESC",
    )?;
    let message_iter = selected.query_map(params![user_id, start, count], |row| ret_mes(row))?;
    let mut messages: Vec<Message> = Vec::new();
    for message in message_iter {
        log::info!("Message: {:?}", (&message));
        messages.push(message?);
    }
    log::info!("All messages loaded to memory");
    Ok(messages)
}

pub fn add_message(message: Message, conn: &Connection) -> Result<()> {
    match conn.execute(
        "INSERT INTO messages (
                  id,
                  user_id,
                  date,
                  message,
                  from_me
                  ) VALUES (?1, ?2, ?3, ?4)",
        params![message.id, message.user_id, message.date, message.message],
    ) {
        Ok(_) => log::info!("message {:} added succsessfully!", message.id),
        Err(e) => {
            log::error!("failed to insert message {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}

pub fn add_my_message(message: Message, conn: &Connection) -> Result<()> {
    match conn.execute(
        "INSERT INTO messages (
                  id,
                  user_id,
                  date,
                  message,
                  from_me
                  ) VALUES (?1, ?2, ?3, ?4)",
        params![message.id, message.user_id, message.date, message.message],
    ) {
        Ok(_) => log::info!("message {:} added succsessfully!", message.id),
        Err(e) => {
            log::error!("failed to insert message {:?}", e);
            return Err(e);
        }
    }
    Ok(())
}
