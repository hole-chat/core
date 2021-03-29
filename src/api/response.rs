use crate::db::types::User as SqliteUser;
use serde_derive::{Deserialize, Serialize};
use crate::db::types::SignKey;
pub type InsertKey = String;
#[derive(Serialize, Deserialize)]
enum ResponseType{
    Error,
    NewMessage,
    UserList,
    NewUser,
    FetchedMessages,
    InstanceCreated,
    InstanceAccepted,

}
#[derive(Serialize, Deserialize)]
enum ErrorType{
    WrongKey
}

#[derive(Serialize, Deserialize)]
struct AppError{
    res_type: ResponseType,
}

// Status of last requested action. Like `Create Instance` or `LoadUsers`
struct ActionStatus{
    
}


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub sign_key: SignKey,
    pub insert_key: InsertKey,
    pub messages_count: u32,
}


#[derive(Serialize, Deserialize)]
pub struct UserList{
    pub users: Vec<User>
}
