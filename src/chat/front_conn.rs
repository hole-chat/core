type Decoded = String;
type Encoded = String;

trait Handler<State> {
    fn process(code: State) -> Message;
    fn send(socket: &WebSocketStream<TcpStream>, msg: Message);
}
struct MessageServer {
    new_message: bool,
    text: String,
}

struct MessageClient {
    message_queue: Vec<Message>,
}

impl MessageServer {
    fn new() -> MessageServer {
        MessageServer {
            new_message: false,
            text: String::from(""),
        }
    }
}

impl MessageClient {
    fn new() -> MessageClient {
        MessageClient {
            message_queue: vec![],
        }
    }
}

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

    Ok(())
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
                println!("{:?}", msg.unwrap().into_text().unwrap());
                sender
                    .send(Message::Text("msg".to_owned()))
                    .await
                    .expect("ooops");
                new_msg = receiver.next();
            }
            None => break,
        }
    }

    Ok(())
}
