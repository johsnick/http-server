use std::net::TcpStream;
use std::io::Write;

use http::BodyType;

pub struct Response {
    body: BodyType,
    stream : TcpStream
}

impl Response {
    pub fn new(stream : TcpStream) -> Response{
        Response{body: BodyType::None, stream: stream}
    }

    pub fn set_body(&mut self, new_body : BodyType) {
        self.body = new_body;
    }

    pub fn send(mut self){
        let (len, body) = match self.body {
            BodyType::String(body) => (body.len(), body.into_bytes()),
            BodyType::Bytes(body) => (body.len(), body),
            BodyType::None => (0, Vec::new())
        };

        self.stream.write(b"HTTP/1.1 200\n").unwrap();
        self.stream.write(b"Date: Mon, 23 May 2005 22:38:34 GMT\n").unwrap();
        self.stream.write(b"Content-Type: text/html; charset=UTF-8\n").unwrap();
        self.stream.write(b"Content-Encoding: UTF-8\n").unwrap();
        self.stream.write(b"Last-Modified: Wed, 08 Jan 2003 23:11:55 GMT\n").unwrap();
        self.stream.write(b"Server: a Server\n").unwrap();
        self.stream.write(b"Accept-Ranges: bytes\n").unwrap();
        self.stream.write(&format!("Content-Length: {}\n", len).as_bytes()).unwrap();
        self.stream.write(b"Connection: close\n\n").unwrap();
        self.stream.write(&body).unwrap();
        self.stream.flush().unwrap();
    }
}
