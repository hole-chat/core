use crate::chat::types::PackedMessage;
use async_std::{
    io,
    net::{TcpListener, TcpStream},
    task,
};
use async_tungstenite::{
    accept_async_with_config, tungstenite::protocol::WebSocketConfig, tungstenite::Message,
    WebSocketStream,
};
use futures::{SinkExt, StreamExt};
use serde_derive::Deserialize;
use std::env;
use std::sync::mpsc::{Receiver, Sender};

type SP = Sender<PackedMessage>;
type RP = Receiver<PackedMessage>;

pub fn listen_client(server_sender: SP, to_client_receiver: RP) -> io::Result<()> {
    task::block_on(connect_to_client(server_sender, to_client_receiver))
}

async fn connect_to_client(server_sender: SP, to_client_receiver: RP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:5948".to_string());

    let listener = TcpListener::bind(&addr).await?;

    if let Ok((stream, _)) = listener.accept().await {
        let ss = server_sender.clone();
        let srm = stream.clone();
        let t1 = task::spawn(accept_client(srm, ss, to_client_receiver));
        //   let t2 = task::spawn(respond_to_client(stream, to_client_receiver));
        t1.await;
        //    t2.await;
    }

    Ok(())
}

async fn respond_to_client(stream: TcpStream, to_client_receiver: RP) -> io::Result<()> {
    while let Ok(res) = to_client_receiver.recv() {
        println!("From Server!:\n {}", res.message);
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

async fn accept_client(
    stream: TcpStream,
    server_sender: SP,
    to_client_receiver: RP,
) -> io::Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let cfg = WebSocketConfig {
        max_send_queue: None,
        max_message_size: Some(67108864),
        max_frame_size: Some(16777216),
    };
    // let ws = accept_async_with_config(stream, Some(cfg.clone()))
    //     .await
    //     .expect("err during the ws handshake");
    // println!("connected to: {}", addr);

    //    let wsr = ws.get_ref().to_owned();
    //    let wsr2 = ws.get_ref().to_owned();

    Ok(())
}

async fn connection_for_receiving(
    wsr: TcpStream,
    to_client_receiver: RP,
    cfg: WebSocketConfig,
) -> io::Result<()> {
    let (sc, _) = (accept_async_with_config(wsr, Some(cfg))
        .await
        .expect("error during the clone handshake"))
    .split()
    .unwrap();

    Ok(())
}

async fn connection_for_sending(
    ws: TcpStream,
    server_sender: SP,
    cfg: WebSocketConfig,
) -> io::Result<()> {
    let (mut sender, mut receiver) = (accept_async_with_config(ws, Some(cfg))
        .await
        .expect("error during the clone handshake"))
    .split();
    /*
        while let Ok(res) = to_client_receiver.recv() {
            //sc.send(Message::Text(res.message.to_string()));
        }
    */
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
