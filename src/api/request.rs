use std::sync::atomic::AtomicU32;
use fcpv2::types::SSK;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

type Id = crate::db::types::Id;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateInstance;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendMessage;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoadUsers;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoadMessages;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddUser;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartApp;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopApp;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateInstanceReq {
    pub req_type: CreateInstance,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartAppReq {
    pub req_type: StartApp,
}

#[derive(Deserialize,Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopAppReq {
    pub(crate) req_type: StopApp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadUsersReq {
    pub req_type: LoadUsers,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageReq {
    pub req_type: SendMessage,
    pub user_id: Id,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadMessagesReq {
    pub req_type: LoadMessages,
    pub user_id: Id,
    pub count: u32,
    pub start_index: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddUserReq {
    pub req_type: AddUser,
    pub name: String,
    pub sign_key: String,
    pub insert_key: String,
}

#[test]
fn request_StoppApp_are_correct() {
    let json: &str = "{'reqType':'StopApp'}";
    //log::info!("{} and {}", json, tjsn);
    //assert_eq!(jsoned, StopAppReq{req_type: StopApp});
    //assert_eq!(json,tjsn);
}
#[test]
fn request_StartApp_are_correct() {}
#[test]
fn request_LoadUsers_are_correct() {}
#[test]
fn request_SendMessage_are_correct() {}
#[test]
fn request_LoadMessages_are_correct() {}
#[test]
fn request_AddUser_are_correct() {}
