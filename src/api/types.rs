use crate::db::types::{Time, Id};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message: String,
    pub date: Time,
    pub id: Id,
    pub from_me: bool,
}
