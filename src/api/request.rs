use fcpv2::types::SSK;
use serde_json::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Address {
    street: String,
    city: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    CreateInstance,
    StartApp,
    StopApp,
    SendMessage,
    LoadUsers,
    LoadMessages,
    AddUser,
}

#[derive(Deserialize, Debug)]
struct CreateInstance {
    req_type: RequestType,
    name: String,
}

#[derive(Deserialize, Debug)]
struct StartApp {
    req_type: RequestType,
}

#[derive(Deserialize, Debug)]
struct StopApp {
    req_type: RequestType,
}
#[derive(Deserialize, Debug)]
struct SendMessage {
    req_type: RequestType,
    user_id: u32,
    message: String,
}
#[derive(Deserialize, Debug)]
struct LoadUsers {
    req_type: RequestType,
}

#[derive(Deserialize, Debug)]
struct LoadMessages {
    req_type: RequestType,
    user_id: u32,
    count: u8,
    start_index: u8,
}


#[derive(Deserialize, Debug)]
struct AddUser {
    req_type: RequestType,
    sign_key: String,
    insert_key: String,
}

