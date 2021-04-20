use super::serv_handler::to_server_sender;
use crate::chat::types::{PackedMessage, RP, SP};
use async_std::task;
use fcpv2::types::traits::FcpParser;
use serde_derive::Deserialize;
use std::env;
use tokio::{
    io::{self, AsyncReadExt},
    net::{tcp::OwnedReadHalf, TcpStream},
};

#[tokio::main]
pub async fn listen_server(client_sender: SP, server_receiver: RP) -> io::Result<()> {
    task::block_on(connect_to_server(client_sender, server_receiver))
}

async fn connect_to_server(client_sender: SP, server_receiver: RP) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9481".to_string());

    let sr = client_sender.clone();
    let stream = TcpStream::connect(&addr)
        .await
        .expect("Unable to connect to FCP");
    let (receiver, sender) = stream.into_split();
    log::info!("Connected to FCP");
    let t = task::spawn(server_responce_getter(receiver, client_sender));
    to_server_sender(sender, server_receiver, sr).await?;
    match t.await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
async fn server_responce_getter(mut receiver: OwnedReadHalf, client_sender: SP) -> io::Result<()> {
    // let mut prev = [0; 1024];
    loop {
        // each freenet responce have an identifier and program will define what to do with request by this identifier
        //TODO create handle_fcp_response function
        let mut buffer = [0; 1024];
        match receiver.read(&mut buffer).await {
            Ok(_) => {
                // if prev != buffer {

                let received = String::from_utf8_lossy(&buffer[..]);

                let req = received.lines().next().unwrap();

                match req {
                    "SSKKeypair" => {
                        log::debug!("parsing keypair: {:?}", &req);
                        let keypair = fcpv2::types::SSKKeypair::parse(&received).unwrap();
                        match &keypair.identifier[..] {
                            "config-SSK" => {
                                log::debug!("got SSKKeypair: {:?}", &keypair);
                                // TODO generating UUID and inserting it into .hole.toml
                                let id = uuid::Uuid::new_v4();
                                let conf = crate::chat::Config {
                                    id: crate::db::types::Id(id),
                                    public_key: keypair.insert_uri.clone(),
                                    private_key: keypair.request_uri.clone(),
                                };
                                let config_str = toml::to_string(&conf).unwrap();

                                log::debug!("create toml config");
                                let config_json =
                                    crate::api::response::ResponseType::InitialConfig {
                                        id: crate::db::types::Id(id),
                                        public_key: keypair.insert_uri.clone(),
                                        private_key: keypair.request_uri.clone(),
                                    };
                                log::debug!("create json config");
                                let config_path = std::path::Path::new(".hole.toml");
                                // writing new data to .hole.toml
                                crate::chat::update_config(&config_path, &config_str).unwrap();
                                log::debug!("rewrite config");
                                client_sender
                                    .send(PackedMessage::ToClient(
                                        serde_json::to_string(&config_json).unwrap(),
                                    ))
                                    .unwrap();
                                log::debug!("sended config to client");
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        log::debug!("unhandled: {}", &req);
                        client_sender
                            .send(PackedMessage::FromFreenet(received.to_string()))
                            .expect("Falied to send message to client thread");
                        log::info!("Sended to client! {}", received.chars().count());
                    }
                }
                // prev = buffer;
                // }
            }
            Err(e) => log::error!("Error: {} ", e),
        }
    }
}

#[derive(Deserialize, Debug)]
struct FrontMsg {
    user_id: u32,
    receiver_id: u32,
    message: String,
    time: String,
}
