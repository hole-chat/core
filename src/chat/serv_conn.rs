use crate::chat::types::PackedMessage;
use crate::fcpv2;
use async_std::{
    io,
    net::{TcpListener, TcpStream},
    task,
};
use async_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use serde_derive::Deserialize;
use std::env;
use std::sync::mpsc::Sender;

type SP = Sender<PackedMessage>;

pub fn listen_server(server_sender: SP) -> io::Result<()> {
    task::block_on(connect_to_server(server_sender))
}

async fn connect_to_server(server_sender: SP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9481".to_string());

    println!("Thats All?!");
    let listener = TcpStream::connect(&addr).await?;
    println!("Listened ");

    Ok(())
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    userID: u32,
    receiverID: u32,
    message: String,
    time: String,
}

async fn accept_server(stream: TcpStream, server_sender: SP) -> io::Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);
    Ok(())
}
