// Minimal Rust app for buildpack smoke test: listen on PORT and respond.
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&bind).expect("bind");
    for stream in listener.incoming().flatten() {
        let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok");
    }
}
