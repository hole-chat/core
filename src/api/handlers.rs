use super::response::User;
use super::response::UserList;
use super::{
    response::{AppStatus, ResponseType},
};
use crate::api::request::Request;
use crate::chat::types::PackedMessage;
use crate::chat::types::SP;
use crate::db::{self, messages, types, users};
use async_std::io::Result;
use fcpv2::client::fcp_types::{ClientPut, ClientHello};
use fcpv2::types::{
    traits::{FcpParser, FcpRequest},
    SSK,
};
use rusqlite::Connection;
use serde_json::json;
use std::time::SystemTime;
use uuid::Uuid;

use crate::db::types::Id;

pub fn start_app(server_sender: SP) -> Result<()> {
    server_sender.send(PackedMessage::ToFreenet(ClientHello::new("start_app_request".to_string(), 2.0).convert())).unwrap();
    Ok(())
    //sending *JSON*, what everything is OK
}

pub fn stop_app(conn: &Connection, server_sender: SP) -> Result<()> {
    std::process::exit(0)
}

pub fn load_users( conn: &Connection, server_sender: SP) -> Result<()> {
    let jsoned_users: Vec<_> = users::load_all_users(conn)
        .unwrap()
        .into_iter()
        .map(|x| x.to_jsonable())
        .collect();
    let users: String = serde_json::to_string(&UserList {
        users: jsoned_users,
    })
    .unwrap();
    let _ = server_sender.send(PackedMessage::ToClient(users)).unwrap();
    Ok(())
}
pub fn send_message(user_id: Id, message: String, conn: &Connection, server_sender: SP) -> Result<()> {
    if let Ok(user_data) = db::users::get_user_by_id(user_id, conn) {
        // Add message to DB
        let key = user_data.insert_key;
        let identifier = &user_data.id.0.to_string()[..];
        let message_id: u32 = user_data.messages_count;
        let db_message = db::types::Message {
            id: message_id,
            date: chrono::offset::Local::now(),
            user_id: Id(uuid::Uuid::parse_str(identifier).expect("failed to parse user ID")),
            message: message.clone(),
            from_me: false,
        };
        let _ = db::messages::add_my_message(db_message, conn).unwrap();
        let fcp_req: String =
            ClientPut::new_default_direct(key, identifier, &message[..]).convert();
        server_sender
            .send(PackedMessage::ToFreenet(fcp_req))
            .unwrap();
        Ok(())
    } else {
        // create error types
        server_sender
            .send(PackedMessage::ToClient(
                json!(super::response::AppError {
                    res_type: super::response::ErrorType::WrongUserId
                })
                .to_string(),
            ))
            .unwrap();
        Ok(())
    }
    //sending FCP request
}

pub fn load_messages(user_id: Id, start_index: u32, count:u32, conn: &Connection, server_sender: SP) -> Result<()> {
    let messages = db::messages::select_n_last_messages(
        user_id,
        start_index,
        count,
        conn,
    )
    .unwrap();
    let jsoned = json!(messages);
    let _ = server_sender.send(PackedMessage::ToClient(jsoned.to_string()));
    Ok(())

    //sending *JSON*
}
// Adding user to DB
pub fn add_user(name: String, insert_key: String, sign_key: String, conn: &Connection, server_sender: SP) -> Result<()> {
    let user = db::types::User {
        id: db::types::Id(Uuid::new_v4()),
        name: name,
        sign_key: sign_key,
        insert_key: SSK::parse(&insert_key[..]).unwrap(),
        messages_count: 0,
    };
    db::users::add_user(user, &conn).unwrap();
    // Sending "Ok" response to client
    server_sender
        .send(PackedMessage::ToClient(
            json!(AppStatus {
                res_type: ResponseType::UserAdded
            })
            .to_string(),
        ))
        .unwrap();
    Ok(())
}
