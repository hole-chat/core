use chrono::prelude::*;
use fcpv2::types::{traits::FcpRequest, SSK};


pub const DB_PATH: &str = "hole.db";

pub type SignKey = String;
pub type InsertKey = SSK;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub sign_key: SignKey,
    pub insert_key: InsertKey,
    pub messages_count: u32,
}

#[derive(Debug)]
pub struct Message {
    pub id: u32,
    pub date: NaiveDateTime,
    pub user_id: u32,
    pub message: String,
}
