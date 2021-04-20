use async_trait::async_trait;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use tokio::io::{self, AsyncBufRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
#[derive(Debug)]
struct Fcp {
    connected: bool,
    stream: TcpStream,
    addr: SocketAddr,
    name: String,
}

struct NodeHello {
    responce: String,
}
#[async_trait]
trait FcpConnection {
    async fn new(addr: &'static str, port: u16, name: String) -> Result<Box<Self>, Box<dyn Error>>;
}

#[async_trait]
impl FcpConnection for Fcp {
    async fn new(addr: &'static str, port: u16, name: String) -> Result<Box<Fcp>, Box<dyn Error>> {
        let socket = SocketAddr::new(IpAddr::V4(addr.parse().unwrap()), port);

        let stream = TcpStream::connect(&socket).await?;
        Ok(Box::new(Fcp {
            connected: true,
            stream: stream,
            addr: socket,
            name: name,
        }))
    }
}

#[async_trait]
trait FCP {
    async fn client_hello(&mut self) -> io::Result<NodeHello>;
}
#[async_trait]
impl FCP for Fcp {
    async fn client_hello(&mut self) -> io::Result<NodeHello> {
        let _ = self
            .stream
            .write_all(("ClientHello\nName=ggg\nExpectedVersion=2.0\nEndMessage\n\n").as_bytes());

        let mut buffer = String::new();
        match self.stream.read_to_string(&mut buffer).await {
            Ok(text) => Ok(NodeHello {
                responce: text.to_string(),
            }),
            Err(e) => Err(e),
        }
    }
}
pub async fn test() -> io::Result<()> {
    println!("WTFAAA");
    /*
    let fcp = Fcp::new("localhost", 9481, "lol".to_string())
        .await
        .unwrap();
    println!("{:?}", fcp);
    */
    let stream = TcpStream::connect("127.0.0.1:9481").await?;
    println!("WTF");

    Ok(())
}

/*
// TODO add error if freenet not connected
impl FcpConnection for Fcp {
    fn new(addr: &'static str, port: u16, name: &'static str) -> Result<&'static Self> {
        let socket = SocketAddr::new(IpAddr::V4(addr.parse().unwrap()), port);
        match TcpStream::connect(&socket) {
            Ok(&mut stream) => {
                let _ = stream.write(
                    (format!(
                        "ClientHello\nName={}\nExpectedVersion=2.0\nEndMessage\n\n",
                        name
                    ),)
                        .as_bytes(),
                )?;
                /*
                 * -- Getting Responce -- *
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).await?;
                let responce = String::from_utf8_lossy(&buffer[..]);
                println!("response");
                */
                Ok(Fcp {
                    connected: true,
                    stream: stream,
                    addr: socket,
                    name: name,
                })
            }
            Err(e) => {
                println!("error: {}", e);
                Error::new(io::ErrorKind::Other, e)
            }
        }
    }
}
*/
/*

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:9481").await?;
    let _ = stream
        .write(("ClientHello\nName=ggg\nExpectedVersion=2.0\nEndMessage\n\n").as_bytes())
        .await?;
    println!("Reading response...");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;
    let received = String::from_utf8_lossy(&buffer[..]);
    print!("<<< {}", received);
    let _ = stream.write(
        ("ClientHello\n
          Name=ggg\n
          ExpectedVersion=2.0\n
          EndMessage\n\n")
            .as_bytes(),
    );
}
*/
