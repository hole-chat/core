use crate::db::types::User as SqliteUser;
use serde_derive::{Deserialize, Serialize};
use tungstenite::http::Response;
use crate::db::types::SignKey;
pub type InsertKey = String;
#[derive(Serialize, Deserialize)]
pub enum ResponseType{
    Error,
    NewMessage,
    UserList,
    NewUser,
    FetchedMessages,
    InstanceCreated,
    InstanceAccepted,
    UserAdded

}
#[derive(Serialize, Deserialize)]
pub enum ErrorType{
    WrongKey,
    FailedToAddUser,
    WrongUserId
       
}

#[derive(Serialize, Deserialize)]
pub struct AppError{
    pub res_type: ErrorType,
}

// Status of last requested action. Like `Create Instance` or `LoadUsers`
#[derive(Serialize, Deserialize)]
pub struct AppStatus{
    pub res_type: ResponseType
}


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub sign_key: SignKey,
    pub insert_key: InsertKey,
    pub messages_count: u32,
}


#[derive(Serialize, Deserialize)]
pub struct UserList{
    pub users: Vec<User>
}
