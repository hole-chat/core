use crate::db::types::SignKey;
use crate::db::types::User as SqliteUser;
use serde_derive::{Deserialize, Serialize};
use tungstenite::http::Response;
pub type InsertKey = String;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ResponseType {
    Error,
    NewMessage,
    NewUser,
    FetchedMessages,
    InstanceCreated,
    InstanceAccepted,
#[serde(rename_all = "camelCase")]
    UserAdded(User),
#[serde(rename_all = "camelCase")]
    InitialConfig {
        id: crate::db::types::Id,
        public_key: fcpv2::types::SSK,
        private_key: fcpv2::types::SSK,
    },
    UserList {
     users: Vec<User>,
    }
}
#[derive(Serialize, Deserialize)]
pub enum ErrorType {
    WrongKey,
    FailedToAddUser,
    WrongUserId,
}

#[derive(Serialize, Deserialize)]
pub struct AppError {
    pub res_type: ErrorType,
}

// Status of last requested action. Like `Create Instance` or `LoadUsers`
#[derive(Serialize, Deserialize)]
pub struct AppStatus {
    pub res_type: ResponseType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub sign_key: SignKey,
    pub insert_key: InsertKey,
    pub messages_count: u32,
}

