use chrono::Local;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::{
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

    let (tx, _rx) = broadcast::channel::<String>(100);

    loop {
        let (socket, addr) = listener.accept().await?;

        println!(
            "[{}] New connection from: {}",
            Local::now().format("%H:%M:%S"),
            addr
        );

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            handle_client(socket, tx, rx).await;
        });
    }

    // Ok(())
}

async fn handle_client(
    mut socket: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    let mut username = String::new();
    reader.read_line(&mut username).await.unwrap();
    let username = username.trim().to_string();

    // send a broadcast that a user has joined
    let join_msg = format!("{} has joined the chat\n", username);
    let join_msg = Message {
        username: username.clone(),
        content: join_msg,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::Broadcast,
    };

    let join_msg_json = serde_json::to_string(&join_msg).unwrap();
    tx.send(join_msg_json).unwrap();

    let mut line = String::new();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                 if result.unwrap() == 0 {
                     break;
                 }
                 let msg = Message {
                     username: username.clone(),
                     content: line.trim().to_string(),
                     timestamp: Local::now().format("%H:%M:%S").to_string(),
                     message_type: MessageType::UserMessage,
                 };
                 let json = serde_json::to_string(&msg).unwrap();
                 tx.send(json).unwrap();

                 // clear the buffer
                 line.clear();
            }
            // handle incoming broadcasts
            result =  rx.recv() => {
                let msg = result.unwrap();
                writer.write_all(msg.as_bytes()).await.unwrap();
                writer.write_all(b"/n").await.unwrap();

            }
        }
    }
    let left_msg = format!("{} has left the chat\n", username);
    let left_msg = Message {
        username: username.clone(),
        content: left_msg,
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::Broadcast,
    };

    let left_msg_json = serde_json::to_string(&left_msg).unwrap();
    tx.send(left_msg_json).unwrap();
    println!("[{}] {} left", Local::now(), username);
}
