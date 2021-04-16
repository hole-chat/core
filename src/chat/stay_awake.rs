use crate::chat::types::PackedMessage;
use async_std::io;
use fcpv2::client::fcp_types::ClientGet;
use fcpv2::types::{traits::FcpRequest, ReturnType, SSK};
use std::sync::mpsc::Sender;

type SP = Sender<PackedMessage>;

pub async fn request_repeater(ss: SP) -> io::Result<()> {
    loop {
        //TODO create a field with tracked users
        let time = std::time::Duration::from_millis(1000);
        std::thread::sleep(time);
        match ss.send(PackedMessage::ToFreenet(
            ClientGet::new_default(SSK{sign_key: "9Zq-H7vg1iN6852rcL3mQQaIfPZODnIJnKyIy1dE6mk".to_string(), decrypt_key: "n-vQibdLXPDMtW7k5ftbR9HVz4Tb184lUc~MiUGHWAM".to_string(),settings: Some("AQACAAE".to_string())},
                                                      "check",
                                                     ReturnType::Direct).convert()
            // message: format!(
            //     "ClientGet\n\
            //      URI=KSK@msg23.txt\n\
            //      Identifier=doesnt_matter?\n\
            //      Verbosity=0\n\
            //      ReturnType=direct\n\
            //      EndMessage\n\n"
            // ),
        )) {
            Ok(_) => {}
            Err(e) => log::error!("{:?}", e),
        }
    }
}
