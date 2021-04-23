use crate::chat::types::{PackedMessage, RP, SP};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

use fcpv2::client::fcp_types::{ClientHello, ClientPut};
use fcpv2::types::{
    traits::{FcpParser, FcpRequest},
    SSK,
};
pub async fn to_server_sender(
    mut sender: OwnedWriteHalf,
    server_receiver: RP,
    client_sender: SP,
) -> io::Result<()> {
    while let Ok(res) = server_receiver.recv() {
        //TODO from_core_to_server_handler
        match res {
            PackedMessage::ToClient(json) => {
                client_sender.send(PackedMessage::FromCore(json)).unwrap();
                log::info!("Message sended to client thread");
            }
            PackedMessage::ToFreenet(req) => {
                let res_type = req.lines().next().unwrap();
                log::debug!("SENDED {}", req);
                sender.write(req.as_bytes()).await?;
                log::info!("Message sended to freenet");
            }
            _ => {},
        }
        /*if res == "STARTAPP!" {
            let _ = sender
                .write((ClientHello::new("name".to_string(), 2.0).convert()).as_bytes())
                .await?;
        } else if res.message.lines().next() == Some("ClientGet") {
            let _ = sender.write(res.message.as_bytes()).await?;
        } else {
            //log::info!("{:?}", res.message);
            let key = SSK::parse("KSK@msg23.txt").unwrap();
            let cp = ClientPut::new_default(key, "msg23.txt", "hello", &res.message[..]).convert();
            let _ = sender.write(cp.as_bytes()).await;
        }*/
    }

    Ok(())
}
