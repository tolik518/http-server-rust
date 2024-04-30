use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use itertools::Itertools;

struct Request {
    method: String,
    path: String,
    version: String,
    headers: Vec<(String, String)>
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("==== Program running ====");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match &stream {
            Ok(_stream) => {
                let mut tcp_stream = &stream.unwrap();

                // create a buffer to read the stream into
                let request = get_client_request_data(&mut tcp_stream);
                let request = parse_request(request.trim());

                if request.path == "/" {
                    tcp_stream
                        .write_all(&*response_builder(200, "abc"))
                        .unwrap();
                } else {
                    tcp_stream
                        .write_all(&*response_builder(404, ""))
                        .unwrap();
                }

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    println!("==== Program finished ====");
}

fn response_builder(code: u16, content: &str) -> Vec<u8> {
    let code_status = match code {
        200 => "OK",
        404 => "Not Found",
        _ => "Internal Server Error"
    };

    let content_length = content.len().to_string();
    let headers = vec![
        ("Content-Type", "text/html"),
        ("Content-Length", &*content_length),
    ];

    let status_line = format!("HTTP/1.1 {code} {code_status}\r\n");

    let response_string = format!(
        "{status_line}{headers}\r\n{content}\r\n",
        status_line = status_line,
        headers = headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}\r\n"))
            .join(""),
        content = content
    );
    println!("Response: ");
    println!("{}", response_string);
    return response_string.as_bytes().to_owned();
}

fn get_client_request_data(tcp_stream: &mut &TcpStream) -> String {
    let mut client_request = String::new();
    let mut buffer: Vec<char> = Vec::new();
    for byte in tcp_stream.bytes() {
        //const ESCAPE_SEQUENCE: [char; 4] = ['\r', '\n', '\r', '\n'];
        if buffer.ends_with(&['\r', '\n', '\r']) {
            client_request = buffer.iter().join("");
            break;
        }

        buffer.push(byte.unwrap() as char);
    }

    return client_request;
}

fn parse_request(client_request: &str) -> Request {
    let mut request = Request {
        method: String::new(),
        path: String::new(),
        version: String::new(),
        headers: Vec::new()
    };

    let mut lines = client_request.lines();
    let first_line = lines.next().unwrap();
    let mut first_line_parts = first_line.split_whitespace();
    request.method = first_line_parts.next().unwrap().to_string();
    request.path = first_line_parts.next().unwrap().to_string();
    request.version = first_line_parts.next().unwrap().to_string();

    for line in lines {
        //split line on :
        let header_parts: Vec<&str> = line.split(": ").collect();
        //push to headers
        request.headers.push((header_parts[0].to_string(), header_parts[1].to_string()));
    }

    return request;
}
