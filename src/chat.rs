use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

const URL: [&str; 4] = [
    "wss://chat.destiny.gg/ws",
    "wss://chat.omniliberal.dev/ws",
    "wss://chat.strims.gg/ws",
    "wss://chat2.strims.gg/ws",
];

pub struct DGG {
    write: futures_util::stream::SplitSink<
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        tokio_tungstenite::tungstenite::Message,
    >,
    read: futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl DGG {
    pub async fn connect() -> Result<Self> {
        let (stream, res) = connect_async(URL[0])
            .await
            .expect("Error connecting to WebSocket");
        match res.status() {
            StatusCode::OK => (),
            _ => (),
        }
        let (write, read) = stream.split();
        Ok(Self { write, read })
    }
    pub async fn handle_stream(&mut self) {
        while let Some(msg) = self.read.next().await {
            let msg = msg.expect("Error reading message");

            if !msg.is_text() {
                continue;
            }

            let msg = msg.into_text().expect("Error Reading Text");

            if let Some((prefix, json)) = msg.split_once(" ") {
                if prefix == "MSG" {
                    let json: Value = serde_json::from_str(json).expect("Error");

                    let nick = json["nick"].as_str().unwrap();
                    let message = json["data"].as_str().unwrap();

                    println!("{}: {}", nick, message);
                }
            }
        }
    }
    pub async fn handle_messages(&mut self) {
        while let Some(msg) = self.read.next().await {
            let msg = msg.expect("Error reading message");

            if !msg.is_text() {
                continue;
            }

            let msg = msg.into_text().expect("Error Reading Text");

            if let Some((prefix, json)) = msg.split_once(" ") {
                if prefix == "MSG" {
                    let json: Value = serde_json::from_str(json).expect("Error");

                    let nick = json["nick"].as_str().unwrap();
                    let message = json["data"].as_str().unwrap();
                    if message.to_lowercase().starts_with("!request") {
                        println!("{}: {}", nick, message);
                    }
                }
            }
        }
    }
}
