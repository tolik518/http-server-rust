mod request;
mod response;
mod status_code;

use std::path::{MAIN_SEPARATOR, Path};
use request::Request;
use response::Response;
use status_code::StatusCode;

use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    println!("==== Server running ====");
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    let mut files_dir: String = "".to_string();

    if args.len() == 3 && args[1] == "--directory" {
        if !args[2].starts_with(MAIN_SEPARATOR) && !args[2].starts_with("C:\\") {
            files_dir = format!("{}{MAIN_SEPARATOR}{}", current_dir, args[2]);
        } else {
            files_dir = args[2].to_string();
        }
    }

    // Wrap files_dir in an Arc and Mutex for thread-safe access, otherwise it wont compile
    let files_dir = Arc::new(Mutex::new(files_dir));

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        // Clone the Arc to pass it into the thread
        let files_dir = Arc::clone(&files_dir);
        thread::spawn(move || {
            let stream = stream.expect("Error in the stream!");

            let mut tcp_stream = &stream;

            let request = Request::from_tcp_stream(&mut tcp_stream);

            match request.path.as_str() {
                "/" => home_action(&mut tcp_stream),
                "/user-agent" => user_agent_action(&mut tcp_stream, &request),
                _ if request.path.starts_with("/echo/") => echo_action(&mut tcp_stream, &request),
                _ if request.path.starts_with("/files/") => directory_action(&mut tcp_stream, &request, &files_dir.lock().unwrap()),
                _ => not_found_action(&mut tcp_stream)
            }
        });
    }

    println!("==== Server finished ====");
}

fn not_found_action(tcp_stream: &mut &TcpStream) {
    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::NotFound)
            .to_bytes())
        .unwrap();
}

fn directory_action(tcp_stream: &mut &TcpStream, request: &Request, files_dir: &str) {
    let file = &request.path
        .split("/")
        .collect::<Vec<&str>>();

    // get only the last element, which is the file name
    let file_name = *file.last().unwrap();

    // Check if the file exists
    let file_path = format!("{}{}{}", files_dir, MAIN_SEPARATOR, file_name);
    if !Path::new(&file_path).exists() {
        return not_found_action(tcp_stream);
    }

    // read file content
    let file_content = std::fs::read_to_string(file_path).unwrap();

    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::Ok)
            .set_content_type("application/octet-stream")
            .set_body(file_content.to_string())
            .to_bytes())
        .unwrap();
}

fn echo_action(tcp_stream: &mut &TcpStream, request: &Request) {
    let echo = &request.path
        .split("/").
        collect::<Vec<&str>>();
    // get only the last element, which we want to echo
    let echo = *echo.last().unwrap();

    tcp_stream
        .write_all(&*Response::new()
            .set_status_code(&StatusCode::Ok)
            .set_body(echo.to_string())
            .to_bytes())
        .unwrap();
}

fn user_agent_action(tcp_stream: &mut &TcpStream, request: &Request) {
    let user_agent = &request.headers
        .iter()
        .find(|(k, _)| k == "User-Agent")
        .unwrap().1;

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

