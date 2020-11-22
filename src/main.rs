mod chat;

mod db;
mod encrypting;
use async_std::io;
use chat::front_conn::listen_client;

use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
    },
    thread,
};
fn main() -> io::Result<()> {
    //    listen_client();

    enum ClientMessage {
        Message(String),
        Nope,
    }
    struct MFA{
        encoded_msg: String
            // TODO add User field
    }; //Message from above


    struct ClientHandler {
        message: ClientMessage,
    }

    struct ServerHandler {
        messages: Vec<MFA>
    }

    let (server_sender, server_receiver) : (Sender<ClientHandler>, Receiver<ClientHandler>) = mpsc::channel(); // server sender, server receiver
    let (client_sender, client_receiver) :  (Sender<ServerHandler>, Receiver<ServerHandler>) = mpsc::channel(); // client sender, client receiver

    let client_listener = thread::spawn(move || {
            let a = server_receiver.recv();
            let it_will_drop = match a.message {
                ClientMessage::Message(m) => {
                    println!("{}", &m);
                    m
                },
                ClientMessage::Nope => String::from("fail!!"),
            };
            server_sender.send(ClientHandler{message: ClientMessage::Message(String::from("I come from frontend"))});
    });

    let server_listener = thread::spawn(move || loop {
        let m1 = String::from("It's a encoded message from Jim");
        let m2 = String::from("It's a encoded message from one killer, who trying to find you");

        let mut fromabove =  ServerHandler{messages: vec![MFA(m1), MFA(m2)]};

        for msg in fromabove.messages.iter() {
            let msg = |MFA(lol)| ;

        }

        let mut a: ClientHandler = ClientHandler {
            message: ClientMessage::Message(String::from("yup")),
        }
        let it_will_drop = match a.message {
            ClientMessage::Message(m) => m,
            ClientMessage::Nope => String::from("fail!!"),
        };
    });
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
