use fcpv2::types::SSK;
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;
use std::sync::atomic::AtomicU32;

use crate::db::types::Id;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum Request {
    StartApp,
    StopApp,
    LoadUsers,
    #[serde(rename_all = "camelCase")]
    SendMessage {
        user_id: Id,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    LoadMessages {
        user_id: Id,
        count: u32,
        start_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    AddUser {
        name: String,
        sign_key: String,
        insert_key: String,
    }, //    CreateInstance TODO v0.3
}

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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
    let json = "{\"type\":\"stopApp\"}";
    let parsed: Request = serde_json::from_str(json).unwrap();
    assert_eq!(Request::StopApp, parsed);
}
#[test]
fn request_StartApp_are_correct() {
    let json = "{\"type\":\"startApp\"}";
    let parsed: Request = serde_json::from_str(json).unwrap();
    assert_eq!(Request::StartApp, parsed);
}
#[test]
fn request_LoadUsers_are_correct() {
    let json = "{\"type\":\"loadUsers\"}";
    let parsed: Request = serde_json::from_str(json).unwrap();
    assert_eq!(Request::LoadUsers, parsed);
}
#[test]
fn request_SendMessage_are_correct() {
    let id = uuid::Uuid::new_v4();
    let json = "{\"type\":\"sendMessage\",\"userId\":\"".to_owned()
        + &id.to_string()[..]
        + "\",\"message\":\"hey jon\"}";
    let parsed: Request = serde_json::from_str(&json).unwrap();
    assert_eq!(Request::SendMessage{user_id: Id(id), message: "hey jon".to_string()}, parsed);
}
#[test]
fn request_LoadMessages_are_correct() {
    let id = uuid::Uuid::new_v4();
    let json = "{\"type\":\"loadMessages\",\"userId\":\"".to_owned()
        + &id.to_string()[..]
        + "\",\"count\":10,\"startIndex\":343}";
    let parsed: Request = serde_json::from_str(&json).unwrap();
    assert_eq!(
        parsed,
        Request::LoadMessages {
            user_id: Id(id),
            count: 10,
            start_index: 343
        }
    );
}
#[test]
fn request_AddUser_are_correct() {
    let json = "{\"type\":\"addUser\",\"name\":\"john\",\"signKey\":\"USK@bxouok43eKpx3g4WmURjviGispWzYxeByiWRsmYOy5k,Y9j~lPDUoNlSTbZfDNaUajfePBrW~KM6uvHyOGWeA7Q,AQECAAE\",\"insertKey\":\"USK@bxouok43eKpx3g4WmURjviGispWzYxeByiWRsmYOy5k,Y9j~lPDUoNlSTbZfDNaUajfePBrW~KM6uvHyOGWeA7Q,AQECAAE\"}";
    let parsed: Request = serde_json::from_str(json).unwrap();
    assert_eq!(parsed, Request::AddUser{
        name: "john".to_string(),
        sign_key: "USK@bxouok43eKpx3g4WmURjviGispWzYxeByiWRsmYOy5k,Y9j~lPDUoNlSTbZfDNaUajfePBrW~KM6uvHyOGWeA7Q,AQECAAE".to_string(),
        insert_key: "USK@bxouok43eKpx3g4WmURjviGispWzYxeByiWRsmYOy5k,Y9j~lPDUoNlSTbZfDNaUajfePBrW~KM6uvHyOGWeA7Q,AQECAAE".to_string(),
    })

}
