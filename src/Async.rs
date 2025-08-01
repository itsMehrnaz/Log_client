use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server is running on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("connectnig to {}", addr);

        tokio::spawn(async move {

            if let Err(e) = handle_client(socket).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

async fn handle_client(mut socket: TcpStream) -> tokio::io::Result<()> {
    socket.write_all(b"welcome to echo server :))))))))))))))))))))))))").await?;

    let (reader, mut writer) = socket.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = buf_reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed == "quit" {
            writer.write_all(b"goodbye").await?;
            break;
        }

        let response = format!("echo :{}\n", trimmed);
        writer.write_all(response.as_bytes()).await?;
    }
    Ok(())
}