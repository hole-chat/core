mod api;
mod chat;
mod db;
mod encrypting;
use async_std::io;
use chat::front_conn::listen_client;
use chat::serv_conn::listen_server;
use chat::types::PackedMessage;
use log;
use serde_derive::Deserialize;
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

    let db = db::start_db().unwrap();

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

        let t = thread::spawn(move || listen_client(ss, cr, db));

        t.join().expect("failed client thread").unwrap();
    });
    server_thread.join().expect("failed to unrap server thread");
    client_thread.join().expect("failed to unrap client thread");
    Ok(())
}
