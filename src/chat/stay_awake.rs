use crate::chat::types::PackedMessage;
use async_std::io;
use fcpv2::client::fcp_types::ClientGet;
use fcpv2::types::{traits::FcpRequest, ReturnType, KEY, SSK, USK};
use std::sync::mpsc::Sender;

type SP = Sender<PackedMessage>;

pub async fn request_repeater(ss: SP) -> io::Result<()> {

    let db = crate::db::start_db().unwrap();
    //    loop {
    //TODO create a field with tracked users
    log::debug!("Request Repeater Started!");
    loop {
        let users: Vec<crate::db::types::User> = crate::db::users::load_all_users(&db).unwrap();
        let time = std::time::Duration::from_millis(1300);
        std::thread::sleep(time);
        log::debug!("enough sleep");
        for user in users {
            let id = user.id.0.to_string();
            let index = user.messages_count + 1;

        match ss.send(PackedMessage::ToFreenet(
            ClientGet::new_default(
                KEY::USK(
                    USK {
                        ssk: user.insert_key,
                        path: format!("{}/{}", &id, &index),
                    }
                ),
                &format!("rec;{};{}", &id, &index)[..], // TODO create Identifier type
                ReturnType::Direct,
            )
            .convert(),
        )) {
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
