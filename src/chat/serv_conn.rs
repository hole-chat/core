use super::serv_handler::to_server_sender;
use crate::api::types::Message as FrontMessage;
use crate::chat::types::{PackedMessage, RP, SP};
use crate::db::users::{increase_messages_count, get_user_by_id};
use crate::db::messages::add_message;
use crate::db::types::{Message as DBMessage, Id};
use async_std::task;
use fcpv2::types::traits::FcpParser;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use regex::Regex;
use serde_derive::Deserialize;
use std::env;
use tokio::{
    io::{self, AsyncReadExt},
    net::{tcp::OwnedReadHalf, TcpStream},
};

#[tokio::main]
pub async fn listen_server(
    client_sender: SP,
    server_receiver: RP,
    conn: Pool<SqliteConnectionManager>,
) -> io::Result<()> {
    task::block_on(connect_to_server(client_sender, server_receiver, conn))
}

async fn connect_to_server(
    client_sender: SP,
    server_receiver: RP,
    conn: Pool<SqliteConnectionManager>,
) -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9481".to_string());

    let sr = client_sender.clone();
    let stream = TcpStream::connect(&addr)
        .await
        .expect("Unable to connect to FCP");
    let (receiver, sender) = stream.into_split();
    log::info!("Connected to FCP");
    let t = task::spawn(server_responce_getter(receiver, client_sender, conn));
    to_server_sender(sender, server_receiver, sr).await?;
    match t.await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
async fn server_responce_getter(
    mut receiver: OwnedReadHalf,
    client_sender: SP,
    conn: Pool<SqliteConnectionManager>,
) -> io::Result<()> {
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
                        /*
                        let id_regex = Regex::new(r"^(\w*-\w*)").unwrap();
                        let identifier_type: String = id_regex
                            .captures(&keypair.identifier[..])
                            .expect("wrong type identifier")[0].to_string();
                        */
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
                    "DataFound" => {
                        let db = conn.get().unwrap();
                        log::debug!("Receive a new message!!! {:?}", &received.trim());
                        let rec = received.clone();
                        let splitted: Vec<&str> = rec.split_inclusive("AllDatan").collect();
                        log::debug!("\n\n\n\n\n AAAAAAAAA \n\n\n\n");
                        match Regex::new("AllData\nIdentifier=(.*)\nCompletionTime=(.*)\nStartupTime=(.*)\nDataLength=(.*)\nGlobal=(.*)\nMetadata.ContentType=(.*)\nData\n((.|\n)*)"){
                            Ok(reg) => {
                        match reg.captures(&received[..]){

                                Some(captured) => {
                        log::debug!("\n\n\n\n\n AAAAAAAAA {:?} \n\n\n\n", captured);
                        let data_length: usize = usize::from_str_radix(&captured[4], 10).unwrap();
                        let message = &captured[7][0..data_length].to_string();
                        let parsed_message = serde_json::from_str::<
                            crate::api::response::FreenetMessage,
                        >(&message[..]);
                        match parsed_message {
                            Ok(json) => {
                                let json_clone = json.clone();
                                let front_message = FrontMessage {
                                    date: json.date,
                                    from_me: false,
                                    id: json.id,
                                    message: json.message,
                                };
                                client_sender
                                    .send(PackedMessage::FromCore(
                                        serde_json::to_string(&front_message).unwrap(),
                                    ))
                                    .unwrap();
                                log::debug!("Send gotted message to frontend...");

                                log::debug!("Adding message to DB...");
                                let user = get_user_by_id(Id(json_clone.clone().id), &db).unwrap();

                                add_message(DBMessage{
                                    id: user.my_messages_count,
                                    date: json_clone.date,
                                    user_id: Id(json_clone.id),
                                    message: json_clone.message,
                                    from_me: false,

                                }, &db).unwrap();

                                log::debug!("Increasing messages count...");
                                increase_messages_count(crate::db::types::Id(json.id), &db).unwrap();


                            }
                            Err(_) => {
                                log::error!("Failed to parse gotted message");
                            }
                        }
                                },
                                None => {log::error!("Failed to parse regexp")},
                            }

                            },
                            Err(e) => {log::error!("Failed to unwrap regexp")}
                        }
                    }
                    "Alldata" => {
                        log::debug!("Receive a new AllData!!! {:?}", &received);
                        let message =
                            fcpv2::node::fcp_response::AllData::parse(&received[..]).unwrap();
                        log::debug!("Parse new message!!!! {:?}", &message);

                        //while (&lines.next() != &Some("AllData")){
                        //   &lines.next();
                        //}
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
