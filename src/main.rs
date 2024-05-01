mod request;
mod response;
mod status_code;

use request::Request;
use response::Response;
use status_code::StatusCode;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use itertools::Itertools;


fn main() {
    println!("==== Program running ====");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = stream.expect("Error in the stream!");

            let mut tcp_stream = &stream;

            let request = Request::from_tcp_stream(&mut tcp_stream);

            match request.path.as_str() {
                "/" => home_action(&mut tcp_stream),
                "/user-agent" => user_agent_action(&mut tcp_stream, &request),
                _ if request.path.starts_with("/echo/") => echo_action(&mut tcp_stream, &request),
                _ => not_found_action(&mut tcp_stream)
            }
        });
    }

    println!("==== Program finished ====");
}

fn not_found_action(tcp_stream: &mut &TcpStream) {
    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::NotFound)
            .to_bytes())
        .unwrap();
}

fn echo_action(tcp_stream: &mut &TcpStream, request: &Request) {
    let echo = &request.path.split("/").collect::<Vec<&str>>();
    let echo = *echo.last().unwrap();
    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::Ok)
            .set_body(echo.to_string())
            .to_bytes())
        .unwrap();
}

fn user_agent_action(tcp_stream: &mut &TcpStream, request: &Request) {
    let user_agent = &request.headers.iter().find(|(k, _)| k == "User-Agent").unwrap().1;
    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::Ok)
            .set_body(user_agent.to_string())
            .to_bytes())
        .unwrap();
}

fn home_action(tcp_stream: &mut &TcpStream) {
    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::Ok)
            .to_bytes())
        .unwrap();
}

