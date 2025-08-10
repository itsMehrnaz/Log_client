mod packet;
use packet::LogPacket;
use bincode;
use tokio::net::{TcpStream};
use tokio::io::{AsyncWriteExt};


//Client
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let packet = LogPacket::new("Info", "connected Now!", "[12:37]");

    let payload = bincode::serialize(&packet).unwrap();

    let len = payload.len() as u32;
    let len_bytes = len.to_be_bytes();

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(&len_bytes).await?;
    stream.write_all(&payload).await?;
    stream.flush().await?;

    Ok(())

}