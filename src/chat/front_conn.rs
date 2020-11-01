use crate::encrypting;
use async_std::{
    io,
    net::{TcpListener, TcpStream},
    task,
};
use async_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use serde_derive::Deserialize;
use std::env;
pub fn listen_client() -> io::Result<()> {
    task::block_on(connect_to_client())
}

async fn connect_to_client() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;

    while let Ok((stream, _)) = listener.accept().await {
        task::spawn(accept_client(stream));
    }
    println!("HEY");

    Ok(())
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    userID: u32,
    receiverID: u32,
    message: String,
    time: String,
}

async fn accept_client(stream: TcpStream) -> io::Result<()> {
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
                    let id = received_msg.receiverID;
                    println!("{:?}", id);

                    /* message example
                    {
                    "userID": 123456789,
                    "receiverID": 123456789,
                    "message": "hey dude",
                    "time": "Tue Oct 13 2020 18:31:22 GMT+0300 (Eastern European Summer Time)",

                    }
                    */
                    sender
                        .send(Message::Text(String::from("msg").to_owned()))
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
