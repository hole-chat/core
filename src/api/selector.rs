use crate::chat::types::{PackedMessage, SP}; use rusqlite;
use super::{
    handlers,
    request::Request,
    response::{AppError, ErrorType, ResponseType},
};
use async_std::io::Result;
use rusqlite::Connection;
use serde_json::from_str;
use serde_json::json;
// server_sender sending data to server thread;
pub fn request_selector(json: &str, server_sender: SP, conn: &Connection) -> Result<()> {
    // if let Ok(res) = from_str::<CreateInstanceReq>(&json) {
    //TODO v0.3 Add Instances     return Ok(());
    // }
    log::info!("matching request...");
    let parsed: Request = serde_json::from_str(json).unwrap();
    match parsed{
        Request::StartApp => {
            handlers::start_app(server_sender.clone())?
        },
        Request::StopApp => {
            handlers::stop_app(&conn, server_sender.clone())?
        },
        Request::LoadUsers => {
            handlers::load_users(&conn, server_sender.clone())?
        },
        Request::SendMessage {
           user_id, message
        } => {
            handlers::send_message(user_id, message, &conn, server_sender.clone())?
        },
        Request::LoadMessages{
           user_id, count, start_index
        } => {
            handlers::load_messages(user_id, start_index, count, &conn, server_sender.clone())?
        },
        Request::AddUser{
            name, sign_key, insert_key
        } => {
            handlers::add_user(name, insert_key, sign_key, &conn, server_sender.clone())?
        },
    }

    /*
    if let Ok(res) = from_str::<StartAppReq>(&json) {kk
        handlers::start_app(res, server_sender.clone())?
    } else if let Ok(res) = from_str::<StopAppReq>(&json) {
        handlers::stop_app(res, conn, server_sender.clone())?
    } else if let Ok(res) = from_str::<LoadUsersReq>(&json) {
        handlers::load_users(res, conn, server_sender.clone())?
    } else if let Ok(res) = from_str::<SendMessageReq>(&json) {
        handlers::send_message(res, conn, server_sender.clone())?
    } else if let Ok(res) = from_str::<LoadMessagesReq>(&json) {
        handlers::load_messages(res, conn, server_sender.clone())?
    } else if let Ok(res) = from_str::<AddUserReq>(&json) {
        match handlers::add_user(res, conn, server_sender.clone()) {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Sending error to user, because failed to add user
                let _ = server_sender
                    .clone()
                    .send(PackedMessage::ToClient(
                        json!(AppError {
                            res_type: ErrorType::FailedToAddUser
                        })
                        .to_string(),
                    ))
                    .unwrap();
                return Ok(());
            }
        }
    } else {
        log::error!("{}\n is wrong formatted", json)
    }
    */

    Err(async_std::io::Error::new(
        async_std::io::ErrorKind::InvalidData,
        serde_json::to_string(&Request::StartApp).unwrap(),
    ))
}
