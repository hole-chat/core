use crate::chat::types::PackedMessage;
use std::sync::{Arc, Mutex};
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

use crate::api::selector::request_selector;

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

        let ss = server_sender.clone();
        log::info!("connected to: {}", addr);
        let (sender, receiver) = ws.split();

        log::debug!("launching repeater...");
        let t1 = task::spawn(connection_for_receiving(
            sender,
            client_receiver,
            client_repeater,
        ));
        let t2 = task::spawn(connection_for_sending(receiver, server_sender, conn));
        let t3 = task::spawn(request_repeater(ss));
        t1.await?;
        t3.await?;
        t2.await?;
    }

    Ok(())
}

async fn connection_for_receiving(
    mut sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    client_receiver: RP,
    server_sender: SP,
) -> io::Result<()> {
    log::info!("Connection for receiving launched");
    //    let mut prev: PackedMessage = PackedMessage::FromFreenet("nothing".to_string());
    while let Ok(res) = client_receiver.recv() {
        //TODO call client get after receiving NodeHello
        // log::debug!("RES {:?}", &res);
        // log::debug!("PREV {:?}", &prev);
        // if res != &prev {
        // prev = res.clone();
        // log::debug!("they are different");
        match res {
            PackedMessage::FromCore(json) => {
                async_std::task::block_on(sender
                    .send(Message::Text(json))).expect("Couldn't send message")

            }
            PackedMessage::FromFreenet(response) => {
                let r = response.clone();
                log::debug!("Got:\n {}", &response);
                async_std::task::block_on(sender
                     // TODO freenet_response_handler
                     .send(Message::Text(r.to_string())));
                //     .await
                //     .expect("Couldn't send messge");
            }
            _ => {}
        }
        // }
    }
    Ok(())
}

async fn connection_for_sending(
    mut receiver: SplitStream<WebSocketStream<TcpStream>>,
    server_sender: SP,
    conn: rusqlite::Connection,
) -> io::Result<()> {
    let ss = server_sender.clone();
    log::info!("Connection for sending launched");
    let mut new_msg = receiver.next();
    loop {
        if let Some(msg) = new_msg.await {
            let jsoned = msg.expect("Falied to unwrap gotted message");
            log::info!("new request");
            match request_selector(&jsoned.to_string()[..], server_sender.clone(), &conn) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e);
                }
            };

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
                return Err(async_std::io::Error::new(
                    async_std::io::ErrorKind::InvalidData,
                    "failed to unwrap message",
                ));
            }
        }
    }
    Ok(())
}
