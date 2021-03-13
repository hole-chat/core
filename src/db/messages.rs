use super::types::Message;
use rusqlite::{params, Connection, Result};

pub fn select_message_by_id(id: u64, conn: &Connection) -> Result<Message> {
    unimplemented!();
}

pub fn select_all_user_message(id: u64, conn: &Connection) -> Result<Vec<Message>> {
    unimplemented!();
}

pub fn select_n_last_messages(user_id: u64, count: u32, conn: &Connection) -> Result<Vec<Message>> {
    unimplemented!();
}

pub fn add_message(message: Message, conn: &Connection) -> Result<()>{
    unimplemented!();
}
