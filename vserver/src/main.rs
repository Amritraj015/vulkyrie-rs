use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::{process::exit, thread};
use vcommon::utilities::{base64_encode, generate_sha1_hash};

static RFC_6455_HANDSHAKE_STRING_TO_APPEND: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

fn main() {
    const SERVER_ADDRESS: &str = "127.0.0.1:7878";
    let listener_creation_result = TcpListener::bind(SERVER_ADDRESS);

    if listener_creation_result.is_err() {
        println!("Failed to start server address - 127.0.0.1:7878");
        exit(1);
    }

    println!("Server started at address: {}", SERVER_ADDRESS);

    for stream_result in listener_creation_result.unwrap().incoming() {
        match stream_result {
            Ok(stream) => {
                thread::spawn(move || handle_connection(stream));
            },
            Err(e) => {
                eprintln!("Error while accepting connection: {}", e);
            },
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .take_while(|result| result.is_ok())
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let websocket_key_header: String = http_request
        .into_iter()
        .filter(|s| s.starts_with("Sec-WebSocket-Key:"))
        .take(1)
        .collect();

    let websocket_key_extraction_result = websocket_key_header.split(" ").last();

    match websocket_key_extraction_result{
        Some(key) => {
            let websoket_key_as_string = format!("{key}{RFC_6455_HANDSHAKE_STRING_TO_APPEND}");
            let key_hash = generate_sha1_hash(&websoket_key_as_string);
            let encoded_key = base64_encode(&key_hash);
            println!("Key: {}, Generated Key: {}", key, encoded_key);
            let response_bytes = format!(
                "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
                encoded_key
            ).into_bytes();
            let _ = stream.write_all(&response_bytes);

            loop {
                let mut msg_buffer = [0; 1024];

                match stream.read(&mut msg_buffer) {
                    Ok(size) if size > 0 => {
                        if let Some(message) = decode_websocket_frame(&msg_buffer, size) {
                            println!("Received message: {}", message);
                            let response = encode_websocket_frame(&message);
                            stream.write_all(&response).expect("Failed to send response");
                        }
                    }
                    _ => break,
                }
            }

            println!("Connection closed");
        },
        None => {
            eprintln!("Error while extracting websocket key");
        }
    }
}

fn decode_websocket_frame(buffer: &[u8], size: usize) -> Option<String> {
    if size < 6 {
        return None;
    }

    let payload_len = (buffer[1] & 0x7F) as usize;
    let mask = &buffer[2..6];
    let mut decoded = vec![0; payload_len];

    for i in 0..payload_len {
        decoded[i] = buffer[6 + i] ^ mask[i % 4];
    }

    String::from_utf8(decoded).ok()
}

fn encode_websocket_frame(message: &str) -> Vec<u8> {
    let mut frame = vec![0x81, message.len() as u8];
    frame.extend_from_slice(message.as_bytes());
    // frame.extend_from_slice(" (From Server)".as_bytes());
    frame
}
