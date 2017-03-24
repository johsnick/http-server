mod request;
mod response;

use std::net::TcpListener;
use std::sync::Arc;
use std::marker::{Send, Sync};
use std::thread;
use http::response::Response;
use http::request::Request;

pub enum BodyType {
    String(String),
    Bytes(Vec<u8>),
    None
}

pub fn start_server<F: 'static>(responder: F) where F: Fn(Request, Response) + Send + Sync {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let responder = Arc::new(responder);
    for stream in listener.incoming() {
        let responder = responder.clone();
        thread::spawn(move ||{
            let mut stream = stream.unwrap();
            let request = Request::parse_request(&mut stream);
            let response = Response::new(stream);
            responder(request, response);
        });
   }
}
