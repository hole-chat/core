use std::sync::atomic::AtomicU32;
use fcpv2::types::SSK;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

type Id = crate::db::types::Id;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CreateInstance;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SendMessage;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LoadUsers;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LoadMessages;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AddUser;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StartApp;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StopApp;

#[derive(Deserialize, Debug)]
pub struct CreateInstanceReq {
    pub req_type: CreateInstance,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct StartAppReq {
    pub req_type: StartApp,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct StopAppReq {
    pub(crate) req_type: StopApp,
}

#[derive(Deserialize, Debug)]
pub struct LoadUsersReq {
    pub req_type: LoadUsers,
}

#[derive(Deserialize, Debug)]
pub struct SendMessageReq {
    pub req_type: SendMessage,
    pub user_id: Id,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct LoadMessagesReq {
    pub req_type: LoadMessages,
    pub user_id: Id,
    pub count: u32,
    pub start_index: u32,
}

#[derive(Deserialize, Debug)]
pub struct AddUserReq {
    pub req_type: AddUser,
    pub name: String,
    pub sign_key: String,
    pub insert_key: String,
}
