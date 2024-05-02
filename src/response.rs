use itertools::Itertools;
use crate::status_code::StatusCode;
pub(crate) struct Response {
    status_code: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String
}

impl Clone for Response {
    fn clone(&self) -> Self {
        Response {
            status_code: self.status_code,
            status_text: self.status_text.clone(),
            headers: self.headers.clone(),
            body: self.body.clone()
        }
    }
}

impl Response {
    pub fn new() -> Self {
        Response {
            status_code: StatusCode::Ok.to_u16(),
            status_text: StatusCode::Ok.to_string(),
            headers: Vec::new(),
            body: "".to_string()
        }
    }

    pub(crate) fn set_status_code(&mut self, status_code: &StatusCode) -> &mut Self {
        self.status_code = status_code.to_u16();
        self.status_text = status_code.to_string();
        self
    }

    pub(crate) fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub(crate) fn set_content_type(&mut self, content_type: &str) -> &mut Self {
        self.headers.push(("Content-Type".to_string(), content_type.to_string()));
        self
    }

    pub(crate) fn set_header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.push((key, value));
        self
    }

    pub(crate) fn set_body(&mut self, body: String) -> &mut Self {
        self.body = body;
        self
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let status_line = format!("HTTP/1.1 {} {}\r\n", &self.status_code, &self.status_text);

        let mut headers = self.headers.clone();

        let content_length = self.body
            .len()
            .to_string();

        if !headers.iter().any(|(k, _)| k == "Content-Type") {
            headers.push(("Content-Type".to_string(), "text/plain".to_string()));
        }

        if !headers.iter().any(|(k, _)| k == "Content-Length") {
            headers.push(("Content-Length".to_string(), content_length));
        }

        let headers = headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}\r\n"))
            .join("");

        let response_string = format!(
            "{status_line}{headers}\r\n{body}",
            status_line = status_line,
            headers = headers,
            body = self.body
        );

        return response_string.as_bytes().to_owned();
    }
}