use std::io::Read;
use std::net::TcpStream;
use itertools::Itertools;

pub(crate) struct Request {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) headers: Vec<(String, String)>
}

impl Request {
    pub fn from_tcp_stream(tcp_stream:&mut &TcpStream) -> Request {
        let client_request = Request::get_client_request_data(tcp_stream);
        Request::parse_request(&client_request)
    }

    fn get_client_request_data(tcp_stream: &mut &TcpStream) -> String {
        let mut client_request = String::new();
        let mut buffer: Vec<char> = Vec::new();
        for byte in tcp_stream.bytes() {
            // why not ['\r', '\n', '\r', '\n']???
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
            let header_parts: Vec<&str> = line.trim().split(": ").collect();
            if header_parts.len() == 2 {
                request.headers.push((
                    header_parts[0].to_string(),
                    header_parts[1].to_string()
                ));
            }
        }

        return request;
    }
}