mod chat;
mod db;
mod encrypting;
mod fcpv2;
use async_std::io;
use chat::front_conn::{listen_client, responding_to_server};
use chat::serv_conn::{listen_server, responding_to_client};
use chat::types::PackedMessage;

use async_std::task;
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
    let (to_server_sender, server_receiver): (Sender<PackedMessage>, Receiver<PackedMessage>) =
        mpsc::channel();
    let (client_sender, client_receiver): (Sender<PackedMessage>, Receiver<PackedMessage>) =
        mpsc::channel();

    let server_thread = thread::spawn(move || {
        let cs = client_sender;
        let sr = server_receiver;
        let cs1 = cs.clone();
        let cs2 = cs.clone();

        let t1 = thread::spawn(move || listen_server(cs1));
        let t2 = thread::spawn(move || responding_to_client(cs2, sr));

        t1.join();
        t2.join();
        // while let Ok(res) = sr.recv() {
        //     println!("From Server:\n {}", res.message);
        // }
    });
    let client_thread = thread::spawn(move || {
        let ss = to_server_sender;
        let cr = client_receiver;
        let ss1 = ss.clone();
        let ss2 = ss.clone();

        let t1 = thread::spawn(move || listen_client(ss1.clone()));
        let t2 = thread::spawn(move || responding_to_server(ss2.clone(), cr));

        t1.join();
        t2.join();
    });
    server_thread.join();
    client_thread.join();
    Ok(())
}

/*
fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    println!("{:?}", &server);
    for stream in server.incoming() {
        spawn(move || {
            println!("{:?}", &stream);
            let mut websocket: WebSocket<std::net::TcpStream> = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}*/
/*
        let (one, two) = keys;

        let value =String::from_utf8_lossy(&*one);
        let strVal = String::from(value);
        let newbytes = strVal.into_bytes();
        print!("{:?}", newbytes);

        let newkey = PrivateKey::import(newbytes);

        Let conn = Connection::open("myfile.db").unwrap();

        conn.execute("CREATE TABLE person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )", NO_PARAMS).unwrap();
        let name: String = "Steve Example".to_string();
        let email: String = "steve@example.org".to_string();
        conn.execute("INSERT INTO person (name, email) VALUES (?1, ?2)",
        &[&name, &email]).unwrap();

}
    */
//let mut std = cli::cli_base::get_stdin();
