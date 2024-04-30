use std::io::{Read, Write};
// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Incoming;
use itertools::Itertools;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("==== Program running ====");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match &stream {
            Ok(_stream) => {
                let mut tcp_stream = stream.unwrap();
                let client_ip = &tcp_stream.peer_addr().unwrap();
                println!("Client ip connected: {client_ip}");
                tcp_stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap()
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    println!("==== Program finished ====");
}
