use std::io;

use chrono::prelude::*;
use fcpv2::types::{traits::{FcpRequest, FcpParser}, SSK};
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Result, ToSql,
};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::response::User as JsonableUser;

pub const DB_PATH: &str = "hole.db";

pub type SignKey = String;
pub type InsertKey = SSK;
#[derive(Debug, Deserialize, Serialize)]
pub struct Id(pub uuid::Uuid);

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub sign_key: SignKey,
    pub insert_key: InsertKey,
    pub messages_count: u32,
}

/// converting SSK to rusqlite type
impl ToSql for Id {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.to_string()))
    }
}

/// converting from rusqlite type to SSK
impl FromSql for Id {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Id> {
        let id = Uuid::parse_str(&(value.as_str()?)[..]).unwrap();
        Ok(Id(id))
    }
}

impl User {
    pub fn to_jsonable(self) -> JsonableUser {
        JsonableUser {
            id: self.id.0.to_string(),
            name: self.name,
            sign_key: self.sign_key,
            insert_key: SSK::convert(&self.insert_key),
            messages_count: self.messages_count,
        }
    }
    pub fn from_jsonable(json: JsonableUser) -> io::Result<User> {
        Ok(User {
            id: Id(Uuid::parse_str(&json.id[..]).unwrap()),
            name: json.name,
            sign_key: json.sign_key,
            insert_key: SSK::parse(&json.insert_key[..]).unwrap(),
            messages_count: json.messages_count,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: u32,
    pub date: NaiveDateTime,
    pub user_id: u32,
    pub message: String,
}
