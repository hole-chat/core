use super::request::*;
use crate::chat::types::SP;
use async_std::io::Result;
use rusqlite::Connection;
pub fn start_app(request: StartAppReq, server_sender: &SP) -> Result<()> {
    Ok(())
}

pub fn stop_app(request: StopAppReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    unimplemented!()
}

pub fn load_users(request: LoadUsersReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    unimplemented!()
}
pub fn send_message(request: SendMessageReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    unimplemented!()
}

pub fn load_messages(
    request: LoadMessagesReq,
    conn: &Connection,
    server_sender: &SP,
) -> Result<()> {
    unimplemented!()
}
pub fn add_user(request: AddUserReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    unimplemented!()
}
