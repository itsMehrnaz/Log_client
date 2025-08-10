mod packet;
use packet::LogPacket;
use bincode;
use tokio::net::{TcpListener, TcpStream};

use tokio::io::{AsyncReadExt, AsyncWriteExt,AsyncBufReadExt, BufReader};

use std::io::{Write, BufWriter};



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

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("connecting to {}", addr);

        tokio::spawn(async move{
            if let Err(e) = handle_client(socket).await {
                eprintln!("error:{}", e);
            }
        });
    }

    async fn handle_client(mut socket: TcpStream) -> tokio::io::Result<()> {
        socket.write_all(b"welcome geeky").await?;

        let(reader, mut writer) = socket.split();
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read: usize = buf_reader.read_line(&mut line).await?;
            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed == "quit" {
                writer.write_all(b"goodbye");
                break;
            }

            let response = format!("echo: {} \n", trimmed);
            writer.write_all(response.as_bytes()).await?;
        }


    Ok(())



    }



    Ok(())




}