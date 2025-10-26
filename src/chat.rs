use std::f64;

use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::Mutex as TokioMutex;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use std::sync::Arc;
use std::collections::HashMap;
use crate::dinkdonk::Movie;

const URL: [&str; 4] = [
    "wss://chat.destiny.gg/ws",
    "wss://chat.omniliberal.dev/ws",
    "wss://chat.strims.gg/ws",
    "wss://chat2.strims.gg/ws",
];
const MAX_OPTIONS: i32 = 50;
const MIN_OPTIONS: i32 = 10;

pub struct DGG {
    write: Option<futures_util::stream::SplitSink<
        WebSocketStream<MaybeTlsStream<TcpStream>>,
        tokio_tungstenite::tungstenite::Message,
    >>,
    read: Option<futures_util::stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    movie_requests: Arc<TokioMutex<HashMap<String, Movie>>>,
}

impl DGG {
    pub fn new() -> Self {
	Self {
	    write: None,
	    read: None,
	    movie_requests: Arc::new(TokioMutex::new(HashMap::new())),
	}
    }
    pub async fn connect(&mut self) -> Result<()> {
        let (stream, res) = connect_async(URL[0])
            .await
            .expect("Error connecting to WebSocket");
        match res.status() {
            StatusCode::OK => (),
            _ => (),
        }
        let (write, read) = stream.split();

	self.write = Some(write);
	self.read = Some(read);

	Ok(())
    }
    pub async fn handle_stream(&mut self) {
	let read = self.read.as_mut().expect("Error reading WebSocket, read is 'None'");
        while let Some(msg) = read.next().await {
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
    //TODO: Add movie request handling. Return Vec<Movie>
    pub async fn handle_messages(&mut self) {
        loop {

	    let Some(read) = self.read.as_mut() else {
		continue;
	    };

	    let Some(next) = read.next().await else {
		// Stream has ended
		break;
	    };

	    let msg = match next {
		Ok(m) => m,
		Err(e) => {
		    eprintln!("Error reading message: {e}");
		    continue;
		}
	    };

	    if !msg.is_text() {
                continue;
	    }

	    let text = match msg.into_text() {
		Ok(t) => t,
		Err(e) => {
		    eprintln!("Error converting message to text: {e}");
		    continue;
		}
	    };

	    if let Some((prefix, json)) = text.split_once(" ") {
		if prefix == "MSG" {
		    let json: Value = serde_json::from_str(json).expect("Error");

		    let nick = json["nick"].as_str().unwrap();
		    let message = json["data"].as_str().unwrap();
		    if message.to_lowercase().starts_with("!request") {
			println!("{}: {}", nick, message);
			if let Some((_first_word, request_text)) = message.split_once(char::is_whitespace) {
			    if let Err(e) = self.handle_request(nick, request_text).await {
				eprintln!("Error handling equest for {nick}: {e}");
			    }
			}
		    }
		}
            }
	}
    }

    async fn handle_request(&mut self, nick: &str, request_text: &str) -> Result<()> {

	let movie_requests = self.movie_requests.lock().await;
	// We don't allow for multiple requests. They must cancel first.
	if movie_requests.contains_key(nick) {
	    return Ok(())
	}

	let api_key = String::new(); // TODO: Figure out how we're going to be handling api keys
	let title = request_text.to_owned();
	let year = None; // TODO: Extract year from request_text.
	let movie = Movie::search_tmdb(api_key, title, year).await?;

	let mut movie_requests = self.movie_requests.lock().await;
	movie_requests.insert(nick.to_string(), movie);

	Ok(())
    }
}
pub fn exp_sat(n: i32) -> i32 {
    if n < MIN_OPTIONS {
        return n;
    }
    let n = n as f64;
    let max = MAX_OPTIONS as f64;
    let min = MIN_OPTIONS as f64;
    (max - (max - min) * f64::consts::E.powf(0.1 - 0.01 * n)).round() as i32
}
