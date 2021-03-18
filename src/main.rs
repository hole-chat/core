mod chat;
mod db;
mod encrypting;
use async_std::io;
use chat::front_conn::listen_client;
use chat::serv_conn::listen_server;
use chat::types::PackedMessage;
use chrono::Utc;
use db::users;
use fcpv2::types::{traits::FcpParser, SSK};
use log;
use simple_logger::SimpleLogger;
use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    },
    thread,
};
/*
                        +-----------------------------------------------------------------------------------+
                        |                                  Client                                           |
                        |                                                                                   |
                        |                                                                                   |
                        |     +----------------------------+          +---------------------------------+   |
                        |     |         Thread 1           |          |           Thread 2              |   |        +-------------------+
+--------------+        |     |  +-----------------------+ |          |  +--------------------------+   |   |        |                   |
|              |        |     |  |        Decode      ----------------------->                      |   |   |        |                   |
|       ---------------------------->                    | |          |  |          to JSON      -------------------------------->       |
|              |        |     |  |                       | |          |  |                          |   |   |        |                   |
|              |        |     |  |                       | |          |  |                          |   |   |        |                   |
|              |        |     |  +-----------------------+ |          |  |                          |   |   |        |                   |
|   Server     |        |     |                            |          |  +--------------------------+   |   |        |       Frontend    |
|              |        |     |                            |          |                                 |   |        |                   |
|              |        |     |  +----------------------+  |          | +-----------------------------+ |   |        |                   |
|              |        |     |  |                      |  |          | |                             | |   |        |                   |
|              |        |     |  |       Encode         |  |          | |                             | |   |        |                   |
|              |        |     |  |            <-----------------------------       from JSON <--------------------------------           |
|              |        |     |  |                      |  |          | |                             | |   |        |                   |
|              |        |     |  +----------------------+  |          | +-----------------------------+ |   |        |                   |
+--------------+        |     |                            |          |                                 |   |        |                   |
                        |     +----------------------------+          +---------------------------------+   |        +-------------------+
                        |                                                                                   |
                        |                                                                                   |
                        +-----------------------------------------------------------------------------------+
*/

fn main() -> io::Result<()> {
    SimpleLogger::new().init().unwrap();
    let conn = db::start_db().unwrap();
    users::add_user(db::types::User{
        id: 9349,
        name: "Nick".to_string(),
        sign_key: "string".to_string(),
        insert_key: fcpv2::types::SSK::parse("SSK@Rgt0qM8D24DltliV2-JE9tYLcrgGAKeDwkz41I3JBPs,p~c8c7FXcJjhcf2vA-Xm0Mjyw1o~xn7L2-T8zlBA1IU").unwrap(),
        messages_count: 1,
    }, &conn);
    let time: chrono::DateTime<chrono::offset::FixedOffset> =
        chrono::DateTime::parse_from_rfc3339("2021-03-18T04:22:42.501Z").unwrap();
    db::messages::add_message(
        db::types::Message {
            user_id: 9349,
            id: 1,
            date: time.naive_utc(),
            message: "hey duude".to_string(),
        },
        &conn,
    )
    .unwrap();
    db::messages::add_message(
        db::types::Message {
            user_id: 9349,
            id: 2,
            date: time.naive_utc(),
            message: "what do you think".to_string(),
        },
        &conn,
    )
    .unwrap();
    db::messages::add_message(
        db::types::Message {
            user_id: 9349,
            id: 3,
            date: time.naive_utc(),
            message: "about that".to_string(),
        },
        &conn,
    )
    .unwrap();

    let (to_server_sender, server_receiver): (Sender<PackedMessage>, Receiver<PackedMessage>) =
        mpsc::channel();

    let (client_sender, client_receiver): (Sender<PackedMessage>, Receiver<PackedMessage>) =
        mpsc::channel();

    let server_thread = thread::spawn(move || {
        let cs = client_sender;
        let sr = server_receiver;

        let t = thread::spawn(move || listen_server(cs, sr));

        t.join().expect("failed server thread").unwrap();
    });

    let client_thread = thread::spawn(move || {
        let ss = to_server_sender;
        let cr = client_receiver;

        let t = thread::spawn(move || listen_client(ss, cr));

        t.join().expect("failed client thread").unwrap();
    });
    server_thread.join().unwrap();
    client_thread.join().unwrap();
    Ok(())
}
