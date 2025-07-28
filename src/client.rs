use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("🔗 Connected to server!");

    stream.write_all(b"سلام سرور جان!\n").await?;
    println!("📤 Message sent");

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("📥 Response: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
