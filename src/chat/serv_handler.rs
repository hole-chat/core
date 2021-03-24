use crate::chat::types::{RP, SP};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

use fcpv2::client::fcp_types::{ClientHello, ClientPut};
use fcpv2::types::{traits::{FcpRequest, FcpParser}, SSK} ;
pub async fn to_server_sender(mut sender: OwnedWriteHalf, server_receiver: RP) -> io::Result<()> {
    while let Ok(res) = server_receiver.recv() {
        //TODO from_core_to_server_handler
        if res.message == "STARTAPP!" {
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
        }
    }

    Ok(())
}
