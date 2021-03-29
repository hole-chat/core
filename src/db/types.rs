use chrono::prelude::*;
use fcpv2::types::{traits::FcpRequest, SSK};
use serde_derive::{Serialize, Deserialize};

use crate::api::response::User as JsonableUser;

pub const DB_PATH: &str = "hole.db";

pub type SignKey = String;
pub type InsertKey = SSK;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub sign_key: SignKey,
    pub insert_key: InsertKey,
    pub messages_count: u32,
}

impl User{
    pub fn to_jsonable(self) -> JsonableUser{
        JsonableUser{
            id: self.id,
            name: self.name,
            sign_key: self.sign_key,
            insert_key: SSK::convert(&self.insert_key),
            messages_count: self.messages_count
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: u32,
    pub date: NaiveDateTime,
    pub user_id: u32,
    pub message: String,
}
