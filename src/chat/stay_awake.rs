use crate::chat::types::PackedMessage;
use async_std::io;
use fcpv2::client::fcp_types::ClientGet;
use fcpv2::types::{traits::FcpRequest, ReturnType, KEY, SSK, USK};
use std::sync::mpsc::Sender;
use std::path::Path;
use std::fs::File;

use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
type SP = Sender<PackedMessage>;

pub async fn request_repeater(ss: SP, conn: Pool<SqliteConnectionManager>) -> io::Result<()> {

    let db = conn.get().unwrap();
    //    loop {
    //TODO create a field with tracked users
    log::debug!("Request Repeater Started!");
    let config: String = String::from_utf8_lossy(&std::fs::read(".hole.toml")?).parse().unwrap();
    let parsed: crate::chat::Config =  toml::from_str(&config[..]).unwrap();
    log::debug!("Config gotted: {:?}", &config);

    loop {
        let users: Vec<crate::db::types::User> = crate::db::users::load_all_users(&db).unwrap();
        let time = std::time::Duration::from_millis(1300);
        std::thread::sleep(time);
        log::debug!("enough sleep");
        for user in users {
            let id = user.id.0.to_string();
            let index = user.messages_count;

            let key = USK {
                        ssk: parsed.private_key.clone(),
                        path: format!("{}/{}", &id, &index + 2),
                    };
            log::debug!("sending {:?}", &key.convert());
        match ss.send(PackedMessage::ToFreenet(
            ClientGet::new_default(
                KEY::USK(
                  key
                ),
                &format!("new-message-{}/{}", &id, &index + 2)[..], // TODO create Identifier type
                ReturnType::Direct,
            )
            .convert(),

        ))
            {
            Ok(_) => {},
            Err(e) => continue ,
      }
        }
    }
}
//}
// message: format!(
//     "ClientGet\n\
//      URI=KSK@msg23.txt\n\
//      Identifier=doesnt_matter?\n\
//      Verbosity=0\n\
//      ReturnType=direct\n\
//      EndMessage\n\n"
// ),
