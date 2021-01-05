use async_trait::async_trait;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{IpAddr, SocketAddr, TcpListener};
struct Fcp {
    connected: bool,
    stream: TcpStream,
    addr: SocketAddr,
    name: String,
}

struct NodeHello {
    responce: String,
}
trait FcpConnection {
    fn new(addr: &'static str, port: u16, name: String) -> Result<Box<Self>, Box<dyn Error>>;
}

impl FcpConnection for Fcp {
    fn new(addr: &'static str, port: u16, name: String) -> Result<Box<Self>, Box<dyn Error>> {
        let socket = SocketAddr::new(IpAddr::V4(addr.parse().unwrap()), port);

        match TcpStream::connect(&socket) {
            Ok(mut stream) => Ok(Box::new(Fcp {
                connected: true,
                stream: stream,
                addr: socket,
                name: name,
            })),
            Err(e) => Err(Box::new(e)),
        }
    }
}
/*
use futures::try_join;
use tokio::io::{self, AsyncBufRead, AsyncReadExt};
use tokio::net::TcpStream;

#[async_trait]
trait FCP {
    async fn connect(&self) -> io::Result<()>;
}
#[async_trait]
impl FCP for Fcp {
    async fn connect(&self) -> io::Result<()> {
        let stream = self.stream;
        let _ =
            stream.write(("ClientHello\nName=ggg\nExpectedVersion=2.0\nEndMessage\n\n").as_bytes());

        let mut buffer = String::new();
        let smth = stream.read_to_string(&mut buffer).await?;
        println!("{:?}", smth);

        Ok(())
        //     }
    }
}
*/

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
