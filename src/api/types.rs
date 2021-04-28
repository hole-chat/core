use crate::db::types::{Time, Id};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct Message {
    pub message: String,
    pub date: Time,
    pub id: uuid::Uuid,
    pub from_me: bool,
}
