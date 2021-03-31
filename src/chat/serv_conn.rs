use super::serv_handler::to_server_sender;
use crate::chat::types::{PackedMessage, RP, SP};
use async_std::task;
use serde_derive::Deserialize;
use std::env;
use tokio::{
    io::{self, AsyncReadExt},
    net::{tcp::OwnedReadHalf, TcpStream},
};

#[tokio::main]
pub async fn listen_server(client_sender: SP, server_receiver: RP) -> io::Result<()> {
    task::block_on(connect_to_server(client_sender, server_receiver))
}

async fn connect_to_server(client_sender: SP, server_receiver: RP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9481".to_string());

    let sr = client_sender.clone();
    let stream = TcpStream::connect(&addr).await.expect("weeror here");
    let (receiver, sender) = stream.into_split();
    let t = task::spawn(server_responce_getter(receiver, client_sender));
    to_server_sender(sender, server_receiver, sr).await?;
    match t.await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
async fn server_responce_getter(mut receiver: OwnedReadHalf, client_sender: SP) -> io::Result<()> {
    loop {
        // each freenet responce have an identifier and program will define what to do with request by this identifier
        //TODO create handle_fcp_response function
        let mut buffer = [0; 1024];
        match receiver.read(&mut buffer).await {
            Ok(_) => {
                let received = String::from_utf8_lossy(&buffer[..]);
                client_sender
                    .send(PackedMessage::FromFreenet(received.to_string()))
                    .expect("Falied to send message to client thread");
                log::info!("Sended to client!");
            }
            Err(e) => log::error!("Error: {} ", e),
        }
    }
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    user_id: u32,
    receiver_id: u32,
    message: String,
    time: String,
}
