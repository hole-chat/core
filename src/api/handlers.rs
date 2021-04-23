use super::response::User;
use super::response::{AppStatus, ResponseType};
use crate::api::request::Request;
use crate::chat::init_config;
use crate::chat::types::PackedMessage;
use crate::chat::types::SP;
use crate::db::{self, messages, types, users};
use async_std::io::Result;
use fcpv2::client::fcp_types::{ClientHello, ClientPut};
use fcpv2::types::{
    traits::{FcpParser, FcpRequest},
    SSK,
};
use rusqlite::Connection;
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use uuid::Uuid;

use crate::db::types::Id;

pub fn start_app(server_sender: SP) -> Result<()> {
    server_sender
        .send(PackedMessage::ToFreenet(
            ClientHello::new("start_app_request".to_string(), 2.0).convert(),
        ))
        .unwrap();
    let config_path = Path::new(".hole.toml");
    match File::open(&config_path) {
        Err(e) => {
            log::debug!("creating new config file...");
            //             std::fs::File::create(&config_path).unwrap();
            server_sender
                .send(PackedMessage::ToFreenet(
                    fcpv2::client::fcp_types::GenerateSSK {
                        identifier: Some("config-SSK".to_string()),
                    }
                    .convert(),
                ))
                .unwrap()
        }
        Ok(res) => {
            let conf = std::fs::read_to_string(&config_path).unwrap();
            log::debug!("Responsing to start_app: {}", &conf);
            let toml: crate::chat::Config = toml::from_str(&conf[..]).unwrap();
            server_sender
                .send(PackedMessage::ToClient(
                    serde_json::to_string(&crate::api::response::ResponseType::InitialConfig {
                        id: toml.id.clone(),
                        public_key: toml.public_key.clone(),
                        private_key: toml.private_key.clone(),
                    })
                    .unwrap(),
                ))
                .unwrap();
            log::debug!("Responsing to start_app");
        } //    TODO converting file from TOML to JSON and sending it to frontend
    };

    Ok(())
    //sending *JSON*, what everything is OK
}

pub fn stop_app(conn: &Connection, server_sender: SP) -> Result<()> {
    std::process::exit(0)
}

pub fn load_users(conn: &Connection, server_sender: SP) -> Result<()> {
    let jsoned_users: Vec<_> = users::load_all_users(conn)
        .unwrap()
        .into_iter()
        .map(|x| x.to_jsonable())
        .collect();
    let users: String = serde_json::to_string(&crate::api::response::ResponseType::UserList {
        users: jsoned_users,
    })
    .unwrap();
    let _ = server_sender.send(PackedMessage::ToClient(users)).unwrap();
    Ok(())
}
pub fn send_message(
    user_id: Id,
    message: String,
    conn: &Connection,
    server_sender: SP,
) -> Result<()> {
    log::debug!("CUM CUM");
    log::debug!("CUM CUM");
    log::debug!("CUM CUM");
    log::debug!("CUM CUM");
    log::debug!("CUM CUM");
    log::debug!("CUM CUM");
    log::debug!("CUM CUM");
    if let Ok(user_data) = db::users::get_user_by_id(user_id, conn) {
        // Add message to DB
        let key = user_data.insert_key;
        let identifier = &user_data.id.0.to_string()[..];
        let message_id: u32 = user_data.my_messages_count;
        let id = Id(uuid::Uuid::parse_str(identifier).expect("failed to parse user ID"));
        let db_message = db::types::Message {
            id: message_id,
            date: chrono::offset::Local::now(),
            user_id: id.clone(),
            message: message.clone(),
            from_me: true,
        };
        let _ = db::messages::add_my_message(db_message, conn).unwrap();
        log::debug!("sending new message to freent");
        let fcp_req: String =
            ClientPut::new_default_direct(fcpv2::types::USK{ ssk: key, path: format!("{}/{}", &identifier, message_id)}, &format!("{}/{}",  &identifier, &message_id )[..], &message[..]).convert();
        server_sender
            .send(PackedMessage::ToFreenet(fcp_req))
            .unwrap();
        let _ = db::users::increase_my_messages_count(id.clone(), conn);
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
    user_id: Id,
    start_index: u32,
    count: u32,
    conn: &Connection,
    server_sender: SP,
) -> Result<()> {
    let messages = db::messages::select_n_last_messages(user_id, start_index, count, conn).unwrap();
    let jsoned = json!(messages);
    let _ = server_sender.send(PackedMessage::ToClient(jsoned.to_string())).unwrap();
    Ok(())

    //sending *JSON*
}
// Adding user to DB
pub fn add_user(
    name: String,
    insert_key: String,
    sign_key: String,
    id: uuid::Uuid,
    conn: &Connection,
    server_sender: SP,
) -> Result<()> {
    let user = db::types::User {
        id: Id(id.clone()),
        name: name.clone(),
        sign_key: sign_key.clone(),
        insert_key: SSK::parse(&insert_key[..]).unwrap(),
        messages_count: 0,
        my_messages_count: 0,
    };
    let user_jsoned = crate::api::response::User{
        id: id.clone().to_string(),
        name: name.clone(),
        sign_key: sign_key.clone(),
        insert_key: insert_key,
        messages_count: 0,
        my_messages_count: 0
    };
    db::users::add_user(user, &conn).unwrap();
    // Sending "Ok" response to client
    //
    //loading all users to frontend
    load_users(conn, server_sender).unwrap();

    // TODO senging only one user to client{
    /*
    server_sender
        .send(PackedMessage::ToClient(
            json!(ResponseType::UserAdded(user_jsoned))
            .to_string(),
        ))
        .unwrap();
*/
    Ok(())
}
