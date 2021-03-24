use crate::chat::types::PackedMessage;
use crate::db;
use async_std::{
    io,
    net::{TcpListener, TcpStream},
    task,
};
use async_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde_derive::Deserialize;
use std::env;
use std::sync::mpsc::{Receiver, Sender};

use super::stay_awake::request_repeater;

use super::types::{RP, SP};

use crate::api::handlers::request_handler;

#[derive(Deserialize, Debug)]
struct FrontMsg {
    user_id: u32,
    receiver_id: u32,
    message: String,
    time: String,
}

pub fn listen_client(
    server_sender: SP,
    client_receiver: RP,
    conn: rusqlite::Connection,
) -> io::Result<()> {
    task::block_on(connect_to_client(server_sender, client_receiver, conn))
}

async fn connect_to_client(
    server_sender: SP,
    client_receiver: RP,
    conn: rusqlite::Connection,
) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:5948".to_string());

    let listener = TcpListener::bind(&addr).await?;

    let client_repeater = server_sender.clone();
    if let Ok((stream, _)) = listener.accept().await {
        let addr = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        log::info!("Peer address: {}", addr);

        let ws = accept_async(stream)
            .await
            .expect("err during the ws handshake");

        log::info!("connected to: {}", addr);
        let (sender, receiver) = ws.split();

        let t1 = task::spawn(connection_for_sending(receiver, server_sender, conn));
        connection_for_receiving(sender, client_receiver, client_repeater).await?;
        t1.await?;
    }

    Ok(())
}

async fn connection_for_receiving(
    mut sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    client_receiver: RP,
    server_sender: SP,
) -> io::Result<()> {
    log::info!("Connection for receiving launched");
    while let Ok(res) = client_receiver.recv() {
        //TODO call client get after receiving NodeHello
        if res.message.lines().next() == Some("NodeHello") {
            let server_sender = server_sender.clone();
            task::spawn(request_repeater(server_sender)).await?;
            log::info!("Client received: \n {}", res.message);
        }

        sender
            .send(Message::Text(String::from(res.message).to_owned()))
            .await
            .expect("couldn't send Message");
    }
    Ok(())
}

async fn connection_for_sending(
    mut receiver: SplitStream<WebSocketStream<TcpStream>>,
    server_sender: SP,
    conn: rusqlite::Connection,
) -> io::Result<()> {
    log::info!("Connection for sending launched");
    let mut new_msg = receiver.next();
    loop {
        if let Some(msg) = new_msg.await {
            let jsoned = msg.expect("Falied to unwrap gotted message");
            request_handler(jsoned.to_string(), &server_sender, &conn);
            /*
            let res: serde_json::Result<FrontMsg> =
                serde_json::from_str(jsoned.to_text().expect("Falied to parse JSON"));
            if let Ok(received_msg) = res {
                let msg = received_msg.message;
                //db::start_db().expect("Failed to start db");

                ss.send(PackedMessage { message: msg }).expect("Falied to send message");
            /* message example
                {
                "user_id": 123456789,
                "receiver_id": 123456789,
                "message": "STARTAPP!",
                "time": "Tue Oct 13 2020 18:31:22 GMT+0300 (Eastern European Summer Time)"
                }
                     */
            } else {
                log::info!("seems, that messsage formatted wrong");
                log::info!("{:?}", res);
            }
            */

            new_msg = receiver.next();
        } else {
            {
                break;
            }
        }
    }
    Ok(())
}
