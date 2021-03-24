use rusqlite;
use crate::chat::types::SP;

use async_std::io::Result;
use rusqlite::Connection;
// server_sender sending data to server thread;
pub async fn request_handler(json: String, server_sender: &SP, conn: &Connection) -> Result<()>{
    unimplemented!()
}
