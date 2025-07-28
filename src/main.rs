use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server is running on 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("new connection from {:?}", addr);

            tokio::spawn(async move{
                let mut buf = [0u8; 1024];

                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(0) => {
                            println!("connection closed by {:?}", addr);
                            return ;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Failed to read from socket; err = {:?}", e);
                            return;
                        }


                    };

                    println!("resived from {:?} : {}", addr, String::from_utf8_lossy(&buf[..n]));

                    if let Err(e) = socket.write_all(&buf[..n]).await {
                        eprintln!("failed to write on socket; err = {:?}", e);
                        return;
                    }
                }
            });
    }
}