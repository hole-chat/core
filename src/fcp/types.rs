use std::error::Error;
use std::io;
use std::net::{IpAddr, SocketAddr, TcpListener};
use tokio::net::TcpStream;
use tokio::prelude::*;
struct Fcp {
    connected: bool,
    stream: TcpStream,
    addr: SocketAddr,
    name: str,
}

trait FcpConnection {
    fn new(addr: str, port: str, name: str) -> Result<Self, Box<dyn Error>>;
}

// TODO add error if freenet not connected
impl Fcp for FcpConnection {
    fn new(addr: str, port: str, name: str) -> Result<Self, Box<dyn Error>> {
        let socket = SocketAddr::new(IpAddr::V4(addr.parse().unwrap()), port);
        match TcpStream::connect(&socket) {
            Ok(mut stream) => {
                let _ = stream
                    .write(
                        (format!(
                            "ClientHello\nName={}\nExpectedVersion=2.0\nEndMessage\n\n",
                            name
                        ),)
                            .as_bytes(),
                    )
                    .await?;
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).await?;
                let responce = String::from_utf8_lossy(&buffer[..]);
                println!("response");
                Ok(stream)
            }
            Error(e) => {
                println!("error: {}", e);
                Error::new(ErrorKind::Other, e)
            }
        }
    }
}
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
