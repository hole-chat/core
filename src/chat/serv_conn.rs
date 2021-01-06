use crate::chat::types::PackedMessage;
use crate::fcpv2;
use async_std::task;
use futures::{SinkExt, StreamExt};
use serde_derive::Deserialize;
use std::env;
use std::sync::mpsc::Sender;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

type SP = Sender<PackedMessage>;

#[tokio::main]
pub async fn listen_server(client_sender: SP) -> io::Result<()> {
    task::block_on(connect_to_server(client_sender))
}

async fn connect_to_server(client_sender: SP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9481".to_string());

    println!("Thats All?!");
    let stream = TcpStream::connect(&addr).await.expect("weeror here");
    println!("stream created");
    let (mut receiver, mut sender) = stream.into_split();
    println!("FSFDSFD");
    let _ = sender
        .write(("ClientHello\nName=ggg\nExpectedVersion=2.0\nEndMessage\n\n").as_bytes())
        .await?;
    loop {
        let mut buffer = [0; 512];
        match receiver.read(&mut buffer).await {
            Ok(s) => {
                let received = String::from_utf8_lossy(&buffer[..]);
                client_sender
                    .send(PackedMessage {
                        message: received.to_string(),
                    })
                    .unwrap();
            }
            Err(e) => println!("Error: {} ", e),
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    userID: u32,
    receiverID: u32,
    message: String,
    time: String,
}

async fn accept_server(stream: TcpStream, client_sender: SP) -> io::Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);
    Ok(())
}
