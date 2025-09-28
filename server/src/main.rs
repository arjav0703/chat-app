use chrono::Local;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    Broadcast,
    UserMessage,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");

    let bind_addr = "127.0.0.1:6969";

    let listener = TcpListener::bind(bind_addr).await?;

    let (_tx, _rx) = broadcast::channel::<String>(100);

    loop {
        let (_socket, addr) = listener.accept().await?;

        println!(
            "[{}] New connection from: {}",
            Local::now().format("%H:%M:%S"),
            addr
        );
    }

    // Ok(())
}
