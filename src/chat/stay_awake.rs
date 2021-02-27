use crate::chat::types::PackedMessage;
use async_std::io;
use std::sync::mpsc::Sender;

type SP = Sender<PackedMessage>;

pub async fn request_repeater(ss: SP) -> io::Result<()> {
    loop {
        let time = std::time::Duration::from_millis(1000);
        std::thread::sleep(time);
        match ss.send(PackedMessage {
            message: format!(
                "ClientGet\n\
                 URI=KSK@msg23.txt\n\
                 Identifier=doesnt_matter?\n\
                 Verbosity=0\n\
                 ReturnType=direct\n\
                 EndMessage\n\n"
            ),
        }) {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        }
    }
}
