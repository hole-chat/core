use crate::chat::types::PackedMessage;
use async_std::io;
use fcpv2::client::fcp_types::ClientGet;
use fcpv2::types::{traits::FcpRequest, ReturnType, KEY, SSK, USK};
use std::sync::mpsc::Sender;

type SP = Sender<PackedMessage>;

pub async fn request_repeater(ss: SP) -> io::Result<()> {
    //    loop {
    //TODO create a field with tracked users
    log::debug!("Request Repeater Started!");
    loop {
        let time = std::time::Duration::from_millis(600);
        std::thread::sleep(time);
        log::debug!("enough sleep");
        match ss.send(PackedMessage::ToFreenet(
            ClientGet::new_default(
                KEY::USK(
                    USK {
                        ssk: SSK {
                            sign_key: "B5CYo9jdAndaZ4IoKdJKCi28bY96f03FhUdY4PO6anY".to_string(),
                            decrypt_key: "9AHiE5ZdMJ9BuIXdv7hucus5VbVtwz9tKjj9LcPbtwM".to_string(),
                            settings: Some("AQACAAE".to_string()),
                        },
                        path: "user-3/0".to_string(),
                    }
                ),
                "check",
                ReturnType::Direct,
            )
            .convert(),
        )) {
            Ok(_) => {},
            Err(e) => continue ,
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
