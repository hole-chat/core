use super::request::*;
use crate::chat::types::SP;
use async_std::io::Result;
use rusqlite::Connection;
use crate::db::{messages, users};
use crate::chat::types::PackedMessage;
pub fn start_app(request: StartAppReq, server_sender: &SP) -> Result<()> {
    Ok(())
        //sending *JSON*, what everything is OK
}

pub fn stop_app(request: StopAppReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    std::process::exit(0)
}

pub fn load_users(request: LoadUsersReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    let users = users::load_all_users(conn);
    unimplemented!();
    //sending *JSON*
}
pub fn send_message(request: SendMessageReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    unimplemented!()
        //sending FCP request
}

pub fn load_messages(
    request: LoadMessagesReq,
    conn: &Connection,
    server_sender: &SP,
) -> Result<()> {
    unimplemented!()
        //sending *JSON*
}
pub fn add_user(request: AddUserReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    unimplemented!()
        //sending *JSON* what user is created
}
