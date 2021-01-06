use crate::chat::types::PackedMessage;
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

pub fn listen_client(server_sender: SP) -> io::Result<()> {
    task::block_on(connect_to_client(server_sender))
}

async fn connect_to_client(server_sender: SP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8989".to_string());

    let listener = TcpListener::bind(&addr).await?;

    println!("Debugging!");
    while let Ok((stream, _)) = listener.accept().await {
        let ss = server_sender.clone();
        task::spawn(accept_client(stream, ss));
    }
    println!("Debugging 2!");

    Ok(())
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    userID: u32,
    receiverID: u32,
    message: String,
    time: String,
}

async fn accept_client(stream: TcpStream, server_sender: SP) -> io::Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws = accept_async(stream)
        .await
        .expect("err during the ws handshake");

    println!("connected to: {}", addr);

    let (mut sender, mut receiver) = ws.split();
    let mut new_msg = receiver.next();
    loop {
        match new_msg.await {
            Some(msg) => {
                let jsoned = msg.unwrap();
                let res: serde_json::Result<FrontMsg> =
                    serde_json::from_str(jsoned.to_text().unwrap());
                if let Ok(received_msg) = res {
                    let msg = received_msg.message;
                    server_sender.send(PackedMessage { message: msg }).unwrap();

                    /* message example
                    {
                    "userID": 123456789,
                    "receiverID": 123456789,
                    "message": "hey dude",
                    "time": "Tue Oct 13 2020 18:31:22 GMT+0300 (Eastern European Summer Time)"
                    }
                    */
                    sender
                        .send(Message::Text(String::from("Sended").to_owned()))
                        .await
                        .expect("ooops");
                } else {
                    println!("seems, that messsage formatted wrong");
                    println!("{:?}", res);
                }

                new_msg = receiver.next();
            }
            None => {
                break;
            }
        }
    }

    Ok(())
}
