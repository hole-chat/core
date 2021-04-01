use crate::chat::types::{PackedMessage, SP};
use rusqlite;

use super::{
    handlers,
    request::*,
    response::{AppError, ErrorType, ResponseType},
};
use async_std::io::Result;
use rusqlite::Connection;
use serde_json::from_str;
use serde_json::json;
// server_sender sending data to server thread;
pub async fn request_selector(json: String, server_sender: &SP, conn: &Connection) -> Result<()> {
    // if let Ok(res) = from_str::<CreateInstanceReq>(&json) {
    //TODO v0.3 Add Instances     return Ok(());
    // }

    if let Ok(res) = from_str::<StartAppReq>(&json) {
        handlers::start_app(res, server_sender)?
    }
    if let Ok(res) = from_str::<StopAppReq>(&json) {
        handlers::stop_app(res, conn, server_sender)?
    }
    if let Ok(res) = from_str::<LoadUsersReq>(&json) {
        handlers::load_users(res, conn, server_sender)?
    }
    if let Ok(res) = from_str::<SendMessageReq>(&json) {
        handlers::send_message(res, conn, server_sender)?
    }
    if let Ok(res) = from_str::<LoadMessagesReq>(&json) {
        handlers::load_messages(res, conn, server_sender)?
    }
    if let Ok(res) = from_str::<AddUserReq>(&json) {
        match handlers::add_user(res, conn, server_sender) {
            Ok(_) => {},
            Err(e) => {
                // Sending error to user, because failed to add user
                let _ = server_sender
                    .send(PackedMessage::ToClient(
                        json!(AppError {
                            res_type: ErrorType::FailedToAddUser
                        })
                        .to_string(),
                    ))
                    .unwrap();
            }
        }
    }
    Err(async_std::io::Error::new(
        async_std::io::ErrorKind::InvalidData,
        "Wrong request",
    ))
}
