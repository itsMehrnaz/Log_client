mod packet;
use packet::LogPacket;
use bincode;
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt,AsyncBufReadExt, BufReader};
use std::io::{Write, BufWriter};



async fn main() -> std::io::Result<()> {

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

        loop {
            let mut len_bufer = [0u8; 4];
            if buf_reader.read_exact(&mut len_bufer).await.is_err(){
                break;
            }
            let len = u32::from_be_bytes(len_bufer);

            let mut payload_bufer = vec![0u8; len as usize];
            buf_reader.read_exact(&mut payload_bufer).await?;

            let packet:LogPacket = bincode::deserialize(&payload_bufer).unwrap();
            println!("Recived Packet: {:?}", packet);

        }
    Ok(())
    }
}