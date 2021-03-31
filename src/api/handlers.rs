use super::{request::*, response::{AppStatus, ResponseType}};
use super::response::User;
use super::response::UserList;
use crate::chat::types::PackedMessage;
use crate::chat::types::SP;
use crate::db::{self, messages, types, users};
use async_std::io::Result;
use fcpv2::types::{traits::FcpParser, SSK};
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
    let user = db::types::User {
        id: db::types::Id(Uuid::new_v4()),
        name: request.name,
        sign_key: request.sign_key,
        insert_key: SSK::parse(&request.insert_key[..]).unwrap(),
        messages_count: 0,
    };
    db::users::add_user(user, &conn).unwrap();
    server_sender.send(PackedMessage::ToClient(json!(AppStatus{res_type: ResponseType::UserAdded}).to_string())).unwrap();
    Ok(())
}
