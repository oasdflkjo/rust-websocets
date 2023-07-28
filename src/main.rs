use serde_json::{Error, Value};
use std::net::TcpListener;
use tungstenite::accept;

#[derive(Debug, serde::Deserialize)]
struct Numbers {
    a: i32,
    b: i32,
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        let mut websocket = accept(stream.unwrap()).unwrap();
        loop {
            let msg = websocket.read_message();
            match msg {
                Ok(m) => {
                    if m.is_text() {
                        let numbers: Result<Numbers, Error> =
                            serde_json::from_str(&m.into_text().unwrap());
                        match numbers {
                            Ok(n) => {
                                let mul = n.a * n.b;
                                websocket
                                    .write_message(tungstenite::Message::Text(mul.to_string()))
                                    .unwrap();
                            }
                            Err(_) => {
                                websocket
                                    .write_message(tungstenite::Message::Text(
                                        "Invalid input".to_string(),
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                }
                Err(tungstenite::Error::ConnectionClosed) => {
                    println!("Client disconnected");
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }
}
