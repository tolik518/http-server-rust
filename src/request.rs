use std::io::Read;
use std::net::TcpStream;
use itertools::Itertools;

pub(crate) struct Request {
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) headers: Vec<(String, String)>,
    pub(crate) body: String
}

impl Request {
    pub fn from_tcp_stream(tcp_stream:&mut &TcpStream) -> Request {
        Request::parse_request(tcp_stream)
    }

    // this function is a complete mess and needs to be refactored very badly
    fn parse_request(tcp_stream: &mut &TcpStream) -> Request {
        let mut client_request = String::new();
        let mut buffer: Vec<char> = Vec::new();
        let mut header_written = false;
        let mut body = String::new();
        let mut current_content_length = 0;
        let mut headers: Vec<(String, String)> = Vec::new();

        for byte in tcp_stream.bytes() {
            let byte = byte.unwrap_or_default();
            let char_byte = byte as char;

            // using ['\r', '\n', '\r', '\n'] doesn't work when nobody is present
            if buffer.ends_with(&['\r', '\n', '\r']) && !header_written {
                header_written = true;
                client_request = buffer.iter().join("");
                buffer.clear();
                for line in client_request.lines() {
                    let header_parts: Vec<&str> = line
                        .trim()
                        .split(": ")
                        .collect();
                    // headers are should always be key-value pairs, #
                    // but we don't want to panic when we don't have 2 parts
                    if header_parts.len() == 2 {
                        headers.push((
                            header_parts[0].to_string(),
                            header_parts[1].to_string()
                        ));
                    }
                }
            } else {
                buffer.push(char_byte);
            }

            // if we have a content-length header, we need to read the body, but only if
            if header_written && headers.iter().any(|(k, _)| k == "Content-Length"){
                // as long as current_content_length is not content_length, we keep reading
                let content_length = headers
                    .iter()
                    .find(|(k, _)| k == "Content-Length")
                    .unwrap().1
                    .parse::<usize>()
                    .unwrap();
                if current_content_length <= content_length {
                    current_content_length += 1;
                    body.push(char_byte);
                }

                if current_content_length == content_length + 1 {
                    break;
                }
            } else if header_written {
                break;
            }
        }
        body = body
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("\n");

        let mut lines = client_request.lines();
        let first_line = lines.next().unwrap();

        let mut first_line_parts = first_line.split_whitespace();
        let method = first_line_parts.next().unwrap().to_string();
        let path = first_line_parts.next().unwrap().to_string();
        let version = first_line_parts.next().unwrap().to_string();

        for line in lines {
            let header_parts: Vec<&str> = line
                .trim()
                .split(": ")
                .collect();
            // headers are should always be key-value pairs, #
            // but we don't want to panic when we dont have 2 parts
            if header_parts.len() == 2 {
                headers.push((
                    header_parts[0].to_string(),
                    header_parts[1].to_string()
                ));
            }
        }

        return Request {
            method,
            path,
            version,
            headers,
            body
        };
    }
}