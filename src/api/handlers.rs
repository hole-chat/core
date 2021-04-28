use super::response::User;
use super::response::{AppStatus, ResponseType};
use crate::api::request::Request;
use crate::api::types::Message as FrontMessage;
use crate::chat::init_config;
use crate::chat::types::PackedMessage;
use crate::chat::types::SP;
use crate::db::{self, messages, types::Message as DbMessage, users};
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
    log::debug!("Sending ClientHello to freenet");
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
            log::debug!("Reading config path... ");
            let conf = std::fs::read_to_string(&config_path).unwrap();
            log::debug!("Parsing config to toml.. ");
            let toml: crate::chat::Config = toml::from_str(&conf[..]).unwrap();

            log::debug!("Sending config to client thread...");
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
    log::debug!("Getting user list from DB... ");
    let jsoned_users: Vec<_> = users::load_all_users(conn)
        .unwrap()
        .into_iter()
        .map(|x| x.to_jsonable())
        .collect();
    log::debug!("Creatin user list JSON... ");
    let users: String = serde_json::to_string(&crate::api::response::ResponseType::UserList {
        users: jsoned_users,
    })
    .unwrap();
    log::debug!("Sending users to client thread... ");
    let _ = server_sender.send(PackedMessage::ToClient(users)).unwrap();
    Ok(())
}
pub fn send_message(
    user_id: Id,
    message: String,
    conn: &Connection,
    server_sender: SP,
) -> Result<()> {
    if let Ok(user_data) = db::users::get_user_by_id(user_id, conn) {
        // Add message to DB
        let key = user_data.insert_key;
        let identifier = &user_data.id.0.to_string()[..];
        let message_id: u32 = user_data.my_messages_count;
        let id = Id(uuid::Uuid::parse_str(identifier).expect("failed to parse user ID"));

        log::debug!("Reading .hole.toml");
        let config: String = String::from_utf8_lossy(&std::fs::read(".hole.toml")?)
            .parse()
            .unwrap();
        log::debug!("Parsing .hole.toml");
        let parsed: crate::chat::Config = toml::from_str(&config[..]).unwrap();
        let my_id = parsed.id.0.to_string();
        let date = chrono::offset::Local::now();
        let db_message = db::types::Message {
            id: message_id,
            date: date.clone(),
            user_id: id.clone(),
            message: message.clone(),
            from_me: true,
        };

        log::debug!("Adding sended message to DB");
        match db::messages::add_my_message(db_message, conn) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to add message to DB");
            }
        }

        let freenet_message = crate::api::response::FreenetMessage {
            id: parsed.id.0, // My identifier
            message: message.clone(),
            date: date.clone(),
        };

        log::debug!("Sending new message to freent...");
        let fcp_req: String = ClientPut::new_default_direct(
            fcpv2::types::USK {
                ssk: key,
                path: format!("{}/{}", &my_id, message_id),
            },
            &format!("new-messge-{}/{}", &identifier, &message_id)[..],
            &serde_json::to_string(&freenet_message).unwrap()[..],
        )
        .convert();
        server_sender
            .send(PackedMessage::ToFreenet(fcp_req))
            .unwrap();
        let _ = db::users::increase_my_messages_count(id.clone(), conn).unwrap();
        Ok(())
    } else {
        // create error types
        log::error!("No such user in DB..");
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
    log::debug!("Loading {} messages from user {:?}...", &count, &user_id);
    let messages: Vec<DbMessage> =
        db::messages::select_n_last_messages(user_id.clone(), start_index, count, conn).unwrap();
    let jsoned = json!(ResponseType::MessageList {
        messages: messages
            .into_iter()
            .map(|msg| -> FrontMessage {
                return FrontMessage {
                    message: msg.message,
                    date: msg.date,
                    id: user_id.0,
                    from_me: msg.from_me,
                };
            })
            .collect(),
        id: user_id.0
    });
    log::debug!("Sending loaded messages to client...");
    let _ = server_sender
        .send(PackedMessage::ToClient(jsoned.to_string()))
        .unwrap();
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
    log::debug!("Retreiving user data from DB...");
    let user = db::types::User {
        id: Id(id.clone()),
        name: name.clone(),
        sign_key: sign_key.clone(),
        insert_key: SSK::parse(&insert_key[..]).unwrap(),
        messages_count: 0,
        my_messages_count: 0,
    };
    let user_jsoned = crate::api::response::User {
        id: id.clone().to_string(),
        name: name.clone(),
        sign_key: sign_key.clone(),
        insert_key: insert_key,
        messages_count: 0,
        my_messages_count: 0,
    };
    log::debug!("Adding new user to DB...");
    db::users::add_user(user, &conn).unwrap();
    // Sending "Ok" response to client
    //
    //loading all users to frontend
    log::debug!("Loading all users to client...");
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
