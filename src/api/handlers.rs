use super::response::User;
use super::response::UserList;
use super::{
    request::*,
    response::{AppStatus, ResponseType},
};
use crate::chat::types::PackedMessage;
use crate::chat::types::SP;
use crate::db::{self, messages, types, users};
use async_std::io::Result;
use fcpv2::client::fcp_types::ClientPut;
use fcpv2::types::{
    traits::{FcpParser, FcpRequest},
    SSK,
};
use rusqlite::Connection;
use serde_json::json;
use uuid::Uuid;

pub fn start_app(request: StartAppReq, server_sender: &SP) -> Result<()> {
    Ok(())
    //sending *JSON*, what everything is OK
}

pub fn stop_app(request: StopAppReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    std::process::exit(0)
}

pub fn load_users(request: LoadUsersReq, conn: &Connection, server_sender: &SP) -> Result<()> {
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
pub fn send_message(request: SendMessageReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    if let Ok(user_data) = db::users::get_user_by_id(request.user_id, conn) {
        let key = user_data.insert_key;
        let identifier = &user_data.id.0.to_string()[..];
        let fcp_req: String =
            ClientPut::new_default_direct(key, identifier, &request.message[..]).convert();
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

pub fn load_messages(
    request: LoadMessagesReq,
    conn: &Connection,
    server_sender: &SP,
) -> Result<()> {
    let messages = db::messages::select_n_last_messages(
        request.user_id,
        request.start_index,
        request.count,
        conn,
    )
    .unwrap();
    let jsoned = json!(messages);
    let _ = server_sender.send(PackedMessage::ToClient(jsoned.to_string()));
    Ok(())

    //sending *JSON*
}
// Adding user to DB
pub fn add_user(request: AddUserReq, conn: &Connection, server_sender: &SP) -> Result<()> {
    let user = db::types::User {
        id: db::types::Id(Uuid::new_v4()),
        name: request.name,
        sign_key: request.sign_key,
        insert_key: SSK::parse(&request.insert_key[..]).unwrap(),
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
