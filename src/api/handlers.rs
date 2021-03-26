use crate::chat::types::SP;
use rusqlite;

use super::request::*;
use async_std::io::Result;
use rusqlite::Connection;
use serde_json::from_str;
use serde_json::json;
// server_sender sending data to server thread;
pub async fn request_handler(json: String, server_sender: &SP, conn: &Connection) -> Result<()> {
    // if let Ok(res) = from_str::<CreateInstanceReq>(&json) {
    //TODO v0.3 Add Instances     return Ok(());
    // }

     if let Ok(res) = from_str::<StartAppReq>(&json) {
        return Ok(());
    }
    if let Ok(res) = from_str::<StopAppReq>(&json) {
        return Ok(());
    }
    if let Ok(res) = from_str::<LoadUsersReq>(&json) {
        return Ok(());
    }
    if let Ok(res) = from_str::<SendMessageReq>(&json) {
        return Ok(());
    }
    if let Ok(res) = from_str::<LoadMessagesReq>(&json) {
        return Ok(());
    }
    if let Ok(res) = from_str::<AddUserReq>(&json) {
        return Ok(());
    }
    Err(async_std::io::Error::new(
        async_std::io::ErrorKind::InvalidData,
        "Wrong request",
    ))
}

#[test]
fn is_making_correct_jsons() {
    let json = from_str::<StopApp>("{\"req_type\":\"StartAppReq\"}").unwrap();
}
