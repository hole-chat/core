use crate::db::types::Id;
use regex::Regex;
use uuid::{self, Uuid};



pub fn parse_message_identifier(identifier: &str) -> (Id, u32) {
         let reg = Regex::new(
            r"new-message-(.*)/(.*)",
        )
        .unwrap();
        let res = reg.captures(identifier).unwrap();
    let uuid = Id(Uuid::parse_str(&res[1]).unwrap());
    let id = u32::from_str_radix(&res[2], 10).unwrap();
    return (uuid, id)

}
