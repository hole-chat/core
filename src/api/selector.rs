use super::{
    handlers,
    request::Request,
    response::{AppError, ErrorType, ResponseType},
};
use crate::chat::types::{PackedMessage, SP};
use async_std::io::Result;
use rusqlite;
use rusqlite::Connection;
use serde_json::from_str;
use serde_json::json;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
// server_sender sending data to server thread;
pub fn request_selector(json: &str, server_sender: SP, conn: Pool<SqliteConnectionManager>,) -> Result<()> {
    // if let Ok(res) = from_str::<CreateInstanceReq>(&json) {
    //TODO v0.3 Add Instances     return Ok(());
    // }
    let conn = conn.get().unwrap();
    log::info!("matching request...");
    let parsed: Request = serde_json::from_str(json).unwrap();
    match parsed {
        Request::StartApp => {
            match handlers::start_app(server_sender.clone()) {
                Ok(_) => return Ok(()),
                Err(_) => {} // Sending error to user, because failed to add user
                             //                let _ = server_sender
                             //                    .clone()
                             //                    .send(PackedMessage::ToClient(
                             //         json!(AppError {
                             //             res_type: ErrorType::FailedToAddUser
                             //         })
                             //         .to_string(),
                             //     ))
                             //     .unwrap();
                             // return Ok(());
            }
        }
        Request::StopApp => match handlers::stop_app(&conn, server_sender.clone()) {
            Ok(_) => {}
            Err(_) => {}
        },
        Request::LoadUsers => match handlers::load_users(&conn, server_sender.clone()) {
            Ok(_) => {}
            Err(_) => {}
        },
        Request::SendMessage { user_id, message } => {
            match handlers::send_message(user_id, message, &conn, server_sender.clone()) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Request::LoadMessages {
            user_id,
            count,
            start_index,
        } => {
            match handlers::load_messages(user_id, start_index, count, &conn, server_sender.clone())
            {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Request::AddUser {
            name,
            sign_key,
            insert_key,
            id
        } => match handlers::add_user(name, insert_key, sign_key, id, &conn, server_sender.clone()) {
            Ok(_) => {}
            Err(_) => {}
        },
        req => {
             log::error!("{}", async_std::io::Error::new(
            async_std::io::ErrorKind::InvalidData,
            serde_json::to_string(&req).unwrap(),
        ))
        }
    }
    Ok(())

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
}
