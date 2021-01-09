use crate::chat::types::PackedMessage;
use crate::fcpv2;
use async_std::task;
use serde_derive::Deserialize;
use std::env;
use std::sync::mpsc::{Receiver, Sender};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}
};

type SP = Sender<PackedMessage>;
type RP = Receiver<PackedMessage>;

#[tokio::main]
pub async fn listen_server(client_sender: SP, server_receiver: RP) -> io::Result<()> {
    task::block_on(connect_to_server(client_sender, server_receiver))
}

async fn connect_to_server(client_sender: SP, server_receiver: RP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9481".to_string());

    let stream = TcpStream::connect(&addr).await.expect("weeror here");
    let (mut receiver, mut sender) = stream.into_split();
    let t = task::spawn(server_responce_getter(receiver, client_sender));
    to_server_sender(sender, server_receiver).await?;
    match t.await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }

}
async fn server_responce_getter(mut receiver: OwnedReadHalf, client_sender: SP) -> io::Result<()>{
    println!("Running");
loop {
        let mut buffer = [0; 512];
        match receiver.read(&mut buffer).await {
            Ok(_) => {
                let received = String::from_utf8_lossy(&buffer[..]);
                println!("received {}", received);
                client_sender
                    .send(PackedMessage {
                        message: received.to_string(),
                    })
                    .unwrap();
            }
            Err(e) => println!("Error: {} ", e),
        }
    }
    //TODO HANDLE ERROR
    Ok(())
}
async fn to_server_sender(mut sender:OwnedWriteHalf, server_receiver: RP) -> io::Result<()>{

    while let Ok(res) = server_receiver.recv(){

        println!("{}", res.message);
        println!("ALIVE\nALIVE\nALIVE\nALIVE\n");
    let _ = sender
        .write(("ClientHello\nName=ggg\nExpectedVersion=2.0\nEndMessage\n\n").as_bytes())
        .await?;
        println!("SENDED\nSENDED\nSENDED\nSENDED\n");
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    user_id: u32,
    receiver_id: u32,
    message: String,
    time: String,
}

